# wasm-xml2js


`wasm-xml2js`is a high-performance WebAssembly (Wasm) library written in Rust for parsing XML strings into JavaScript objects. This library offers an efficient solution for handling XML data in web applications, harnessing the power of Rust and WebAssembly for optimal performance.

## Features

- **Lightning-fast XML parsing**: Leverage Rust's speed and WebAssembly's near-native performance.
- **JavaScript object conversion**: Seamlessly transform XML data into easy-to-use JavaScript objects.
- **Comprehensive XML support**: Handle attributes, text nodes, and nested elements with ease.
- **TypeScript-friendly**: Includes TypeScript definitions for enhanced developer experience.

## Installation

Install the library using npm:

```bash
npm install wasm-xml2js
```

## Usage

Here's a quick example of how to use `parse_string_promise`:

```javascript
import { parse_string_promise } from 'wasm-xml2js';

async function parseXML() {
  const xmlString = `
    <root>
      <item id="1">
        <name>Example Item</name>
        <price>19.99</price>
      </item>
    </root>
  `;

  try {
    const result = await parse_string_promise(xmlString);
    console.log(result);
  } catch (error) {
    console.error('Error parsing XML:', error);
  }
}

parseXML();
```

### TypeScript Usage

For TypeScript users, you can define interfaces for your expected output:

```typescript
import { parse_string_promise } from 'wasm-xml2js';

interface Item {
  id: string;
  name: string;
  price: string;
}

interface RootObject {
  root: {
    item: Item;
  };
}

async function parseXML() {
  const xmlString = `...`; // Your XML string here

  try {
    const result = await parse_string_promise<RootObject>(xmlString);
    console.log(result.root.item.name);
  } catch (error) {
    console.error('Error parsing XML:', error);
  }
}
```

## API Reference

### `parse_string_promise(xmlString: string): Promise<any>`

Parses an XML string and returns a Promise that resolves to a JavaScript object.

- **Parameters:**
  - `xmlString`: The XML string to parse.
- **Returns:** A Promise that resolves to the parsed JavaScript object.
- **Throws:** An error if the XML is invalid or parsing fails.
