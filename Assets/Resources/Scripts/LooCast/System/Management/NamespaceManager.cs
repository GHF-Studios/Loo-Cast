﻿using System;
using UnityEngine;

namespace LooCast.System.Management
{
    using LooCast.System.Identification;
    
    public sealed class NamespaceManager : InternalManager
    {
        #region Static Properties
        public static NamespaceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[NamespaceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<NamespaceManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static NamespaceManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private NamespaceRegistry namespaceRegistry;
        #endregion

        #region Methods
        public void RegisterNamespace(INamespace @namespace)
        {
            namespaceRegistry.Register(@namespace.NamespaceIdentifier, @namespace);
        }

        public INamespace GetNamespace(INamespaceIdentifier namespaceIdentifier)
        {
            return (INamespace)namespaceRegistry.Get(namespaceIdentifier);
        }

        public INamespace GetNamespace(NamespaceIdentifier namespaceIdentifier)
        {
            return GetNamespace(namespaceIdentifier);
        }
        #endregion

        #region Overrides
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            #region Namespace/Type/Instance Registration
            TypeManager typeManager = TypeManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            looCastNamespace = new Namespace("LooCast");
            looCastType = new Type(typeof(NamespaceManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);
            #endregion
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();

            #region Registry Registration
            RegistryManager registryManager = RegistryManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            IType keyType = typeManager.GetType(new TypeIdentifier("LooCast.System.Identification:INamespaceIdentifier"));
            IType valueType = typeManager.GetType(new TypeIdentifier("LooCast.System.Identification:INamespaceIdentifiable"));
            namespaceRegistry = new NamespaceRegistry(keyType, valueType);
            registryManager.RegisterRegistry(namespaceRegistry);
            #endregion
        }
        #endregion
    }
}