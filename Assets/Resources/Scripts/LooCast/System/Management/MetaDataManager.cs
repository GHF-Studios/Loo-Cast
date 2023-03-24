using System;
using UnityEngine;

namespace LooCast.System.Management
{
    using LooCast.System.MetaData;
    using LooCast.System.Identification;
    using LooCast.System.Registration;
    using LooCast.System.Types;

    public sealed class MetaDataManager : InternalManager
    {
        #region Static Properties
        public static MetaDataManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[MetaDataManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<MetaDataManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static MetaDataManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private IMetaDataRegistry<IMetaDataIdentifier, IMetaDataIdentifiable> dataRegistry;
        #endregion

        #region Callbacks
        private void OnPreInitialize()
        {
            string rootResourceFolderPath = global::System.IO.Path.Combine(MainManager.Instance.RootPersistentPath, "Resources");

            
        }
        #endregion

        #region Methods
        public void RegisterMetaData(IMetaData data)
        {
            dataRegistry.Register(data.MetaDataIdentifier, data);
        }

        public IMetaData GetMetaData(IMetaDataIdentifier dataIdentifier)
        {
            return (IMetaData)dataRegistry.Get(dataIdentifier);
        }
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            RegisterPreInitializationAction(OnPreInitialize);
        }

        public override void InitializeInstance()
        {
            base.InitializeInstance();

            #region Namespace/Type/Instance Registration
            TypeManager typeManager = TypeManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            IUnityInstanceType dataManagerType = (IUnityInstanceType)typeManager.GetType("LooCast.System.Management:MetaDataManager");
            
            UnityInstance dataManagerInstance = new UnityInstance(this, dataManagerType);

            unityInstanceManager.RegisterUnityInstance(dataManagerInstance);
            #endregion
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();

            #region Registry Registration
            TypeManager typeManager = TypeManager.Instance;
            RegistryManager registryManager = RegistryManager.Instance;

            IType iMetaDataIdentifierType = typeManager.GetType("LooCast.System.Identification:IMetaDataIdentifier");
            IType iMetaDataIdentifiableType = typeManager.GetType("LooCast.System.Identification:IMetaDataIdentifiable");
            
            dataRegistry = new MetaDataRegistry(iMetaDataIdentifierType, iMetaDataIdentifiableType);

            registryManager.RegisterRegistry(dataRegistry);
            #endregion
        }
        #endregion
    }
}