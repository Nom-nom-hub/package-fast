import { getPackageInfo } from '../src/index';

describe('Package Fast Core Functions', () => {
  test('should return package info with default version', () => {
    const pkg = getPackageInfo('test-package');
    expect(pkg.name).toBe('test-package');
    expect(pkg.version).toBe('1.0.0');
  });
});