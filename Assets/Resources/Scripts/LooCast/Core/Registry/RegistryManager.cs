using System;
using System.Collections.Generic;

namespace LooCast.Core.Registry
{
    using Identifier;
    using UnityEngine;

    public class RegistryManager : SubModuleManager
    {
        #region Static Properties
        public static RegistryManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[RegistryManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<RegistryManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static RegistryManager instance;
        #endregion

        #region Properties
        public override Manager[] SubManagers => subManagers;
        #endregion

        #region Fields
        private Dictionary<TypeIdentifier, object> registries;
        private Manager[] subManagers;
        #endregion

        #region Methods
        public override void PreInitialize()
        {
            registries = new Dictionary<TypeIdentifier, object>();
            subManagers = new Manager[]
            {
                
            };
        }

        public override void Initialize()
        {

        }

        public override void PostInitialize()
        {

        }
        
        public void RegisterRegistry<KeyType, ValueType>(Registry<KeyType, ValueType> registry) where KeyType : IIdentifier where ValueType : IIdentifiable
        {
            TypeIdentifier registryTypeIdentifier = (TypeIdentifier)registry.Identifier;
            if (!MainManager.IsPreInitializing)
            {
                throw new Exception("[RegistryManager] Registries can only be registered in Pre-Initialization!");
            }
            if (registries.ContainsKey(registryTypeIdentifier))
            {
                throw new Exception($"[RegistryManager] Registry '{registryTypeIdentifier}' already exists!");
            }

            registries.Add(registryTypeIdentifier, registry);
        }

        public Registry<KeyType, ValueType> GetRegistry<KeyType, ValueType>(TypeIdentifier registryTypeIdentifier) where KeyType : IIdentifier where ValueType : IIdentifiable
        {
            if (!MainManager.IsPreInitialized)
            {
                throw new Exception("[RegistryManager] Registries can only be accessed after Pre-Initialization!");
            }
            if (!registries.ContainsKey(registryTypeIdentifier))
            {
                throw new Exception($"[RegistryManager] Registry '{registryTypeIdentifier}' does not exist!");
            }

            return (Registry<KeyType, ValueType>)registries[registryTypeIdentifier];
        }
        #endregion
    }
}
