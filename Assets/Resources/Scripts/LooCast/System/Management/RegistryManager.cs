using System;
using UnityEngine;
using System.Collections.Generic;

namespace LooCast.System.Management
{
    using LooCast.System.Identification;
    
    public sealed class RegistryManager : InternalManager
    {
        #region Static Properties
        public static RegistryManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[RegistryManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
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
        #endregion

        #region Fields
        
        private Dictionary<IRegistryIdentifier, IRegistryIdentifiable> registries;
        #endregion

        #region Methods
        public void RegisterRegistry(IRegistryIdentifiable registry)
        {
            IRegistryIdentifier registryIdentifier = registry.RegistryIdentifier;
            if (registries.ContainsKey(registryIdentifier))
            {
                throw new Exception($"[RegistryManager] Registry '{registryIdentifier}' already exists!");
            }

            registries.Add(registryIdentifier, registry);
        }

        public IRegistry<KeyType, ValueType> GetRegistry<KeyType, ValueType>(IRegistryIdentifier registryIdentifier) where KeyType : IIdentifier where ValueType : IIdentifiable
        {
            if (registries.ContainsKey(registryIdentifier))
            {
                return (IRegistry<KeyType, ValueType>)registries[registryIdentifier];
            }
            else
            {
                throw new Exception($"[RegistryManager] Registry '{registryIdentifier}' does not exist!");
            }
        }

        public IRegistry<IIdentifier, IIdentifiable> GetRegistry(RegistryIdentifier registryIdentifier)
        {
            return GetRegistry(registryIdentifier);
        }
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            registries = new Dictionary<IRegistryIdentifier, IRegistryIdentifiable>();
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;
            
            looCastNamespace = namespaceManager.GetNamespace("LooCast");
            looCastType = new Type(typeof(RegistryManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);
            #endregion
        }
        #endregion
    }
}
