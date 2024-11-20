export function toTitleCase(str: string): string {
    const newStr = str.replace(/_/g, ' ')
    return newStr.slice(0, 1).toUpperCase() + newStr.slice(1).toLowerCase()
}
