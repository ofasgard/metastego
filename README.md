# metastego

This is a little tool that encodes or decodes a payload using a technique I'm calling metasteganography for lack of a better word. Unlike steganography, the payload is not actually stored inside the image (or whatever binary file you wish to use). Instead, the image is used to encode the payload and produce an obfuscated blob that can only be decoded if you have access to the original image.

This is done by creating an oracle of byte values to corresponding offsets within the image. For example, if `0x00` appears at offset `1358` in the file, then you would encode `0x00` as `1358`. As long as you have access to the original image, you can look up these offsets and reconstruct the original payload.

## Usage

To encode a payload using a specific image (or other binary):

```sh
$ metastego encode payload.bin payload_encoded.bin smile.jpg
```

To decode an encoding payload using the same image:

```sh
$ metastego decode payload_encoded.bin payload_decoded.bin smile.jpg
```
## Disclaimer

This is not encryption. It's just an unusual encoding scheme, intended as a proof of concept for payload obfuscation and environmental keying. It's an experiment in obfuscating data in a way that is not well signatured and is sensitive to the local environment (i.e. is a certain image or binary present).

I'm aware you could achieve a similar effect by hashing the image and using the hash as an encryption key. I think there's value in avoiding encryption and decryption of payloads if you don't need to do so, from a stealth and opsec perspective.
