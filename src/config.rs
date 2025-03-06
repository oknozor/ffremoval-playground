1. Locate all files where `FEATURE_X_ENABLED` is defined or used.

2. In each file, remove or comment out the corresponding code. For example:

    - If it's a variable assignment:
      ```javascript
      // const FEATURE_X_ENABLED = true;
      ```

    - If it's a function call:
      ```javascript
      // if (FEATURE_X_ENABLED) {
      //   // code for the feature
      // }
      ```

    - If it's an import:
      ```javascript
      // import FeatureX from './FeatureX';
      ```

3. After making these changes, run your test suite to ensure there are no unexpected issues.

4. Commit and push the changes with a meaningful commit message like "Remove feature flag FEATURE_X_ENABLED".