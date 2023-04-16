using LooCast.System.Identifiers;
using System;

namespace LooCast.System.Registries
{
    public class MainRegistry : Registry<TypeIdentifier, IRegistry>
    {
        #region Methods
        /// <summary>
        /// Tries to get the registry for the given managedCSSystemType.
        /// </summary>
        /// <param name="managedCSSystemType">The type that is managed by the registry that you are trying to get</param>
        /// <returns>The registry, which manages the given managedCSSystemType</returns>
        public bool TryGetRegistry(global::System.Type managedCSSystemType, out IRegistry registry)
        {
            return TryGetValue(managedCSSystemType, out registry);
        }

        public IRegistry GetRegistry(global::System.Type managedCSSystemType)
        {
            if (TryGetRegistry(managedCSSystemType, out IRegistry registry))
            {
                return registry;
            }
            throw new global::System.Exception($"[MainRegistry] Registry of type '{managedCSSystemType}' not found!");
        }
        #endregion
    }
}
