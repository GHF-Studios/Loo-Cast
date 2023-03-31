using System;
using UnityEngine;

namespace LooCast.System.Managers
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Registries;

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
                    UnityEngine.GameObject.DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = MainManager.Instance.GameObjectInstance.transform;
                    instance = new TypeManager();
                }
                return instance;
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

        #region Constructors
        public TypeManager() : base("LooCast.System.Managers:TypeManager", MainManager.Instance)
        {
            
        }
        #endregion

        #region Methods
        public void RegisterType(Type type)
        {
            typeRegistry.Add(type.TypeIdentifier, type);
        }

        public Type GetType(TypeIdentifier typeIdentifier)
        {
            if (!typeRegistry.TryGetValue(typeIdentifier, out Type type))
            {
                throw new Exception("Type not found.");
            }
            return type;
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