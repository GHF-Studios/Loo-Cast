using LooCast.System.Identifiers;
using System;

namespace LooCast.System.Registries
{
    using LooCast.System.Types;
    
    public class MainRegistry : Registry<ITypeIdentifier, IRegistry>
    {
        #region Properties
        public NamespaceRegistry NamespaceRegistry { get; private set; }
        public TypeRegistry TypeRegistry { get; private set; }
        public SystemObjectRegistry SystemObjectRegistry { get; private set; }
        public GameObjectRegistry GameObjectRegistry { get; private set; }
        public ComponentRegistry ComponentRegistry { get; private set; }
        #endregion

        #region Overrides
        public override void PostInitialize()
        {
            base.PostInitialize();

            NamespaceRegistry = GetRegistry(typeof(INamespace)) as NamespaceRegistry;
            TypeRegistry = GetRegistry(typeof(IType)) as TypeRegistry;
            SystemObjectRegistry = GetRegistry(typeof(ISystemObjectType.ISystemObject)) as SystemObjectRegistry;
            GameObjectRegistry = GetRegistry(typeof(IGameObjectType.IGameObject)) as GameObjectRegistry;
            ComponentRegistry = GetRegistry(typeof(IComponentType.IComponent)) as ComponentRegistry;
        }
        #endregion

        #region Methods
        /// <summary>
        /// Tries to get the registry for the given managedCSSystemType.
        /// </summary>
        /// <param name="managedCSSystemType">The type that is managed by the registry that you are trying to get</param>
        /// <returns>The registry, which manages the given managedCSSystemType</returns>
        public bool TryGetRegistry(Type managedCSSystemType, out IRegistry registry)
        {
            return TryGetValue(managedCSSystemType, out registry);
        }

        public IRegistry GetRegistry(Type managedCSSystemType)
        {
            if (TryGetRegistry(managedCSSystemType, out IRegistry registry))
            {
                return registry;
            }
            throw new Exception($"[MainRegistry] Registry of type '{managedCSSystemType}' not found!");
        }
        #endregion
    }
}
