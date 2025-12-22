// TypeScript Sample - Testing Akari Color Palette
// Open this file in VSCode with Akari theme to see all colors in action

/**
 * This file demonstrates all color tokens used in the Akari theme.
 * Colors are assigned based on semantic meaning and visual hierarchy.
 */

// ============================================================================
// 1. IMPORTS & NAMESPACES (bright_blue #6F879A)
// ============================================================================

import { EventEmitter } from 'events';
import type { RequestHandler } from 'express';

namespace Utils {
  export const VERSION = "1.0.0";
  export const MAX_RETRIES = 3;
}

// ============================================================================
// 2. TYPE DEFINITIONS (bright_cyan #8FB8B0)
// ============================================================================

interface UserConfig {
  id: number;              // property: cyan #6F8F8A
  name: string;            // property: cyan #6F8F8A
  readonly email: string;  // readonly: bright_red #D65A3A
  metadata?: Record<string, unknown>;
}

type ApiResponse<T> = {    // type parameter: bright_magenta #9A8FB3 (italic)
  data: T;
  status: number;
  message: string;
};

enum Status {              // enum: bright_cyan #8FB8B0
  Pending,                 // enum member: bright_red #D65A3A
  Active,
  Completed,
  Failed
}

class DataService<T> {     // type parameter: bright_magenta #9A8FB3
  private static cache = new Map<string, unknown>();  // static: italic
  
  constructor(
    private readonly config: UserConfig  // parameter: foreground #E6DED3, italic
  ) {}
  
  // ============================================================================
  // 3. FUNCTIONS & METHODS (bright_green #8FC57A)
  // ============================================================================
  
  async fetchData(endpoint: string): Promise<T> {
    // keyword: lantern #E26A3B
    const url = `https://api.example.com/${endpoint}`;  // string: lantern #E26A3B
    const response = await fetch(url);  // builtin: bright_yellow #F08A5D
    
    console.log('Data fetched successfully');  // builtin: bright_yellow #F08A5D
    return response.json();
  }
  
  /**
   * @deprecated Use fetchData instead
   * decorator: bright_yellow #F08A5D
   */
  @deprecated
  async getData(id: number): Promise<T> {  // number: bright_red #D65A3A
    const result = await this.fetchData(`/data/${id}`);
    return result;
  }
}

// ============================================================================
// 4. CONSTANTS & NUMBERS (bright_red #D65A3A)
// ============================================================================

const MAX_SIZE = 1000;      // constant name: bright_red #D65A3A
const PI = 3.14159;         // number value: bright_red #D65A3A
const TIMEOUT = 5000;

const CONFIG = {
  apiKey: "secret-key-123",
  retries: 3,
  timeout: 30000,
  enabled: true
} as const;

// ============================================================================
// 5. VARIABLES & PROPERTIES (foreground #E6DED3 / cyan #6F8F8A)
// ============================================================================

let counter = 0;            // variable: foreground #E6DED3
const items: string[] = []; // variable: foreground #E6DED3

const user = {
  name: "Alice",           // property key: cyan #6F8F8A
  age: 30,
  settings: {
    theme: "dark",
    notifications: true
  }
};

// ============================================================================
// 6. CONTROL FLOW & KEYWORDS (lantern #E26A3B)
// ============================================================================

function processData(data: unknown): void {
  if (typeof data === "string") {     // keyword: lantern #E26A3B
    console.log(data.toUpperCase());
  } else if (typeof data === "number") {
    console.log(data * 2);
  } else {
    throw new Error("Invalid data type");  // error: red #C84C32
  }
  
  try {
    // Process data
    for (const item of items) {      // keyword: lantern #E26A3B
      console.log(item);
    }
  } catch (error) {
    console.error("Processing failed", error);
  } finally {
    console.log("Cleanup complete");
  }
}

// ============================================================================
// 7. ASYNC/AWAIT & PROMISES (lantern #E26A3B)
// ============================================================================

async function fetchUser(id: number): Promise<UserConfig> {
  const response = await fetch(`/api/users/${id}`);
  
  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }
  
  return await response.json();
}

// ============================================================================
// 8. GENERICS & TYPE PARAMETERS (bright_magenta #9A8FB3, italic)
// ============================================================================

function createPair<T, U>(first: T, second: U): [T, U] {
  return [first, second];
}

class Container<T extends object> {
  constructor(private value: T) {}
  
  getValue(): T {
    return this.value;
  }
}

// ============================================================================
// 9. DECORATORS & METADATA (bright_yellow #F08A5D)
// ============================================================================

@sealed
class Greeter {
  @logged
  greet(name: string): string {
    return `Hello, ${name}!`;
  }
}

function sealed(constructor: Function) {
  Object.seal(constructor);
  Object.seal(constructor.prototype);
}

function logged(target: any, key: string, descriptor: PropertyDescriptor) {
  console.log(`Method ${key} was called`);
}

// ============================================================================
// 10. COMMENTS (comment #7A7A75, italic)
// ============================================================================

// Single line comment
/* Multi-line comment
   spanning several lines */

/**
 * JSDoc documentation comment
 * @param id User identifier
 * @returns User configuration object
 */

// TODO: Implement caching
// FIXME: Handle edge cases
// NOTE: This is important

// ============================================================================
// 11. REGULAR EXPRESSIONS & SPECIAL STRINGS
// ============================================================================

const emailPattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;  // regexp: bright_green #8FC57A
const phonePattern = /\d{3}-\d{4}-\d{4}/;

const template = `
  Name: ${user.name}
  Age: ${user.age}
  Status: ${Status.Active}
`;

// ============================================================================
// 12. BUILTIN OBJECTS & FUNCTIONS (bright_yellow #F08A5D)
// ============================================================================

const now = Date.now();
const json = JSON.stringify({ data: "test" });
const parsed = JSON.parse(json);

Math.random();
Math.floor(3.14);
parseInt("42");
parseFloat("3.14");

Array.from([1, 2, 3]);
Object.keys(user);
Promise.resolve(42);

// ============================================================================
// 13. COMPARISON: Similar Colors Side by Side
// ============================================================================

namespace API {              // bright_blue #6F879A
  interface Response {       // bright_cyan #8FB8B0
    data: {
      userId: number;        // property: cyan #6F8F8A
      userName: string;      // property: cyan #6F8F8A
    }
  }
}

// Question: Can you easily distinguish namespace, interface, and property?
// This is the area where color separation could be improved.

export { DataService, UserConfig, Status, Utils };

