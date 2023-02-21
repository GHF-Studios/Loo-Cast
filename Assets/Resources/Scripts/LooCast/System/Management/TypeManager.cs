using System;
using UnityEngine;

namespace LooCast.System.Management
{
    using LooCast.System.Identification;
    
    public class TypeManager : InternalManager
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
        private Registry<ITypeIdentifier, ITypeIdentifiable> typeRegistry;
        #endregion

        #region Methods
        public void RegisterType(IType type)
        {
            typeRegistry.Register(type.TypeIdentifier, type);
        }

        public IType GetType(ITypeIdentifier typeIdentifier)
        {
            return (IType)typeRegistry.Get(typeIdentifier);
        }

        public IType GetType(TypeIdentifier typeIdentifier)
        {
            return GetType((ITypeIdentifier)typeIdentifier);
        }
        #endregion

        #region Overrides
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;

            looCastNamespace = namespaceManager.GetNamespace("LooCast");
            looCastType = new Type(typeof(TypeManager), looCastNamespace);
            looCastUnityInstance = new Instance(this, looCastType);

            RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastUnityInstance);
            #endregion
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();

            #region Registry Registration
            RegistryManager registryManager = RegistryManager.Instance;
            IType keyType = GetType(new TypeIdentifier("LooCast.System.Identification:ITypeIdentifier"));
            IType valueType = GetType(new TypeIdentifier("LooCast.System:IType"));
            typeRegistry = new Registry<ITypeIdentifier, ITypeIdentifiable>(keyType, valueType);
            registryManager.RegisterRegistry(typeRegistry);
            #endregion
        }
        #endregion
    }
}