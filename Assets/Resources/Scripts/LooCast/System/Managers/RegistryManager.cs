using System;
using UnityEngine;
using System.Collections.Generic;

namespace LooCast.System.Managers
{
    using global::LooCast.System.Identifiers;
    
    public sealed class RegistryManager : InternalManager
    {
        #region Static Properties
        public static RegistryManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new RegistryManager();
                    instance.UnityEngineGameObject.name = "[RegistryManager]";
                    instance.UnityEngineGameObject.layer = 31;
                    instance.UnityEngineGameObject.tag = "INTERNAL";
                    instance.UnityEngineGameObject.transform.parent = LooCast.Instance.gameObject.transform;
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static RegistryManager instance;
        #endregion

        #region Constructors
        public RegistryManager() : base("LooCast.System.Managers.RegistryManager", MainManager.Instance)
        {
            registries = new Dictionary<SystemObjectIdentifier, Registry<Identifier, ILooCastObject>>();
        }
        #endregion

        #region Properties
        #endregion

        #region Fields
        private Dictionary<SystemObjectIdentifier, Registry<Identifier, ILooCastObject>> registries;
        #endregion

        #region Methods
        public void RegisterRegistry(Registry<Identifier, ILooCastObject> registry)
        {
            SystemObjectIdentifier registryIdentifier = registry.RegistryIdentifier;
            if (registries.ContainsKey(registryIdentifier))
            {
                throw new Exception($"[RegistryManager] Registry '{registryIdentifier}' already exists!");
            }
            registries.Add(registryIdentifier, registry);
        }

        public Registry<Identifier, ILooCastObject> GetRegistry(SystemObjectIdentifier registryIdentifier)
        {
            if (!registries.TryGetValue(registryIdentifier, out Registry<Identifier, ILooCastObject> registry))
            {
                throw new Exception($"[RegistryManager] Registry '{registryIdentifier}' could not be found!");
            }
            return registry;
        }
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();
        }

        public override void InitializeInstance()
        {
            base.InitializeInstance();
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();
        }
        #endregion
    }
}
