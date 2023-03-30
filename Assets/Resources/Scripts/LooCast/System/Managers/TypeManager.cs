﻿using System;
using UnityEngine;

namespace LooCast.System.Managers
{
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;

    public sealed class TypeManager : InternalManager
    {
        #region Static Properties
        public static TypeManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[TypeManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<TypeManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static TypeManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private TypeRegistry typeRegistry;
        #endregion

        #region Methods
        public void RegisterType(Type type)
        {
            typeRegistry.Register(type.TypeIdentifier, type);
        }

        public Type GetType(ITypeIdentifier typeIdentifier)
        {
            return (Type)typeRegistry.Get(typeIdentifier);
        }

        public Type GetType(TypeIdentifier typeIdentifier)
        {
            return GetType(typeIdentifier);
        }
        #endregion

        #region Overrides
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            looCastNamespace = namespaceManager.GetNamespace("LooCast");
            looCastType = new Type(typeof(TypeManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);
            #endregion
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();

            #region Registry Registration
            RegistryManager registryManager = RegistryManager.Instance;
            IType keyType = GetType(new TypeIdentifier("LooCast.System.Identifiers:ITypeIdentifier"));
            IType valueType = GetType(new TypeIdentifier("LooCast.System.Identifiers:ITypeIdentifiable"));
            typeRegistry = new TypeRegistry(keyType, valueType);
            registryManager.RegisterRegistry(typeRegistry);
            #endregion
        }
        #endregion
    }
}