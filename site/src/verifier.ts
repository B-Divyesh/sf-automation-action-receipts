import canonicalize from 'canonicalize';

export type Receipt = Record<string, any>;

export type Verification = {
  valid: boolean;
  chainValid: boolean;
  signatureValid: boolean;
  eventCount: number;
  receiptId?: string;
  bundleSha256?: string;
  message: string;
};

const FORMAT = 'https://actionreceipts.dev/receipt/v1';
const encoder = new TextEncoder();

function hex(bytes: ArrayBuffer): string {
  return [...new Uint8Array(bytes)].map((value) => value.toString(16).padStart(2, '0')).join('');
}

function fromBase64(value: string): Uint8Array<ArrayBuffer> {
  const binary = atob(value);
  const bytes = new Uint8Array(new ArrayBuffer(binary.length));
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return bytes;
}

async function digest(value: unknown): Promise<string> {
  return hex(await crypto.subtle.digest('SHA-256', encoder.encode(canonicalize(value))));
}

function eventPayload(event: Receipt) {
  return {
    sequence: event.sequence,
    timestamp: event.timestamp,
    kind: event.kind,
    tool: event.tool,
    command: event.command ?? null,
    input: event.input,
    output: event.output,
    exit_code: event.exit_code ?? null,
    artifacts: event.artifacts,
    previous_hash: event.previous_hash,
  };
}

function signingPayload(receipt: Receipt) {
  return {
    format: receipt.format,
    receipt_id: receipt.receipt_id,
    created_at: receipt.created_at,
    updated_at: receipt.updated_at,
    state: receipt.state,
    subject: receipt.subject,
    policy: receipt.policy,
    events: receipt.events,
    chain_head: receipt.chain_head,
  };
}

function invalid(receipt: Receipt, message: string, chainValid = false): Verification {
  return {
    valid: false,
    chainValid,
    signatureValid: false,
    eventCount: Array.isArray(receipt?.events) ? receipt.events.length : 0,
    receiptId: typeof receipt?.receipt_id === 'string' ? receipt.receipt_id : undefined,
    message,
  };
}

export async function verifyReceipt(receipt: Receipt): Promise<Verification> {
  if (!receipt || typeof receipt !== 'object' || receipt.format !== FORMAT) {
    return invalid(receipt, 'Unsupported or missing Action Receipts v1 format.');
  }
  if (!Array.isArray(receipt.events)) return invalid(receipt, 'The receipt event list is missing.');
  if (receipt.state !== 'sealed' || !receipt.proof) return invalid(receipt, 'This receipt is open and has no signature.');
  if (receipt.proof.algorithm !== 'Ed25519') return invalid(receipt, 'Unsupported signature algorithm.');

  let previous = '0'.repeat(64);
  for (let index = 0; index < receipt.events.length; index += 1) {
    const event = receipt.events[index];
    if (event.sequence !== index + 1) return invalid(receipt, `Event ${index + 1} has an invalid sequence.`);
    if (event.previous_hash !== previous) return invalid(receipt, `Event ${index + 1} is not linked to the previous event.`);
    const expected = await digest(eventPayload(event));
    if (event.hash !== expected) return invalid(receipt, `Event ${index + 1} changed after it was recorded.`);
    previous = event.hash;
  }
  if (receipt.chain_head !== previous) return invalid(receipt, 'The chain head does not match the final event.');

  const payload = encoder.encode(canonicalize(signingPayload(receipt)));
  const bundleSha256 = hex(await crypto.subtle.digest('SHA-256', payload));
  if (receipt.proof.bundle_sha256 !== bundleSha256) return invalid(receipt, 'The signed bundle digest does not match.', true);
  try {
    const publicKey = await crypto.subtle.importKey('raw', fromBase64(receipt.proof.public_key), { name: 'Ed25519' }, false, ['verify']);
    const signatureValid = await crypto.subtle.verify('Ed25519', publicKey, fromBase64(receipt.proof.signature), payload);
    if (!signatureValid) return invalid(receipt, 'The Ed25519 signature does not match.', true);
  } catch {
    return invalid(receipt, 'This browser could not verify the Ed25519 signature. Use the CLI verifier.', true);
  }
  return {
    valid: true,
    chainValid: true,
    signatureValid: true,
    eventCount: receipt.events.length,
    receiptId: receipt.receipt_id,
    bundleSha256,
    message: 'Every event hash and the Ed25519 signature are valid.',
  };
}
