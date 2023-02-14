using System;
using UnityEngine;
using System.Collections.Generic;

namespace LooCast
{
    public class RegistryManager : Manager
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

        #region Fields
        private Dictionary<RegistryIdentifier, Registry<IIdentifier, IIdentifiable>> registries;
        #endregion

        #region Methods
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            registries = new Dictionary<RegistryIdentifier, Registry<IIdentifier, IIdentifiable>>();
        }

        public void RegisterRegistry(Registry<IIdentifier, IIdentifiable> registry)
        {
            RegistryIdentifier registryIdentifier = registry.RegistryIdentifier;
            if (registries.ContainsKey(registryIdentifier))
            {
                throw new Exception($"[RegistryManager] Registry '{registryIdentifier}' already exists!");
            }

            registries.Add(registryIdentifier, registry);
        }

        public Registry<IIdentifier, IIdentifiable> GetRegistry(RegistryIdentifier registryIdentifier)
        {
            if (registries.ContainsKey(registryIdentifier))
            {
                return registries[registryIdentifier];
            }
            else
            {
                throw new Exception($"[RegistryManager] Registry '{registryIdentifier}' does not exist!");
            }
        }
        #endregion
    }
}
