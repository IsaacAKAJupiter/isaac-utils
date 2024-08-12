export function extendObject(
    deep: boolean,
    ...objects: { [key: string]: any }[]
) {
    const extended: { [key: string]: any } = {};

    // Merge the object into the extended object
    var merge = function (obj: { [key: string]: any }) {
        for (var prop in obj) {
            if (Object.prototype.hasOwnProperty.call(obj, prop)) {
                // If deep merge and property is an object, merge properties
                if (
                    deep &&
                    Object.prototype.toString.call(obj[prop]) ===
                        '[object Object]'
                ) {
                    extended[prop] = extendObject(
                        deep,
                        extended[prop],
                        obj[prop]
                    );
                } else {
                    extended[prop] = obj[prop];
                }
            }
        }
    };

    // Loop through each object and conduct a merge
    for (let i = 0; i < objects.length; i++) {
        merge(objects[i]);
    }

    return extended;
}
