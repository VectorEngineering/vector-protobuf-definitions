/**
 * Validates if a string is a valid URL
 * @param urlString - The string to validate
 * @returns boolean indicating if the string is a valid URL
 */
export const isValidUrl = (urlString: string): boolean => {
    try {
        new URL(urlString);
        return true;
    } catch (_) {
        return false;
    }
}; 