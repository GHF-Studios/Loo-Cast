using System;
using UnityEngine;

namespace LooCast.System.Management
{
    using LooCast.System.Data;
    using LooCast.System.Identification;
    using LooCast.System.Registration;
    using LooCast.System.Types;

    public sealed class DataManager : InternalManager
    {
        #region Static Properties
        public static DataManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[DataManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<DataManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static DataManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private IDataRegistry<IDataIdentifier, IDataIdentifiable> dataRegistry;
        private IDataObjectRegistry<IDataObjectIdentifier, IDataObjectIdentifiable> dataObjectRegistry;
        private IDataFileRegistry<IDataFileIdentifier, IDataFileIdentifiable> dataFileRegistry;
        private IDataFolderRegistry<IDataFolderIdentifier, IDataFolderIdentifiable> dataFolderRegistry;
        #endregion

        #region Methods
        public void RegisterData(IData data)
        {
            dataRegistry.Register(data.DataIdentifier, data);
        }

        public IData GetData(IDataIdentifier dataIdentifier)
        {
            return (IData)dataRegistry.Get(dataIdentifier);
        }

        public void RegisterDataObject(IDataObject dataObject)
        {
            dataObjectRegistry.Register(dataObject.DataObjectIdentifier, dataObject);
            RegisterData(dataObject);
        }

        public IDataObject GetDataObject(IDataObjectIdentifier dataObjectIdentifier)
        {
            return (IDataObject)dataObjectRegistry.Get(dataObjectIdentifier);
        }

        public void RegisterDataFile(IDataFile dataFile)
        {
            dataFileRegistry.Register(dataFile.DataFileIdentifier, dataFile);
            RegisterData(dataFile);
        }

        public IDataFile GetDataFile(IDataFileIdentifier dataFileIdentifier)
        {
            return (IDataFile)dataFileRegistry.Get(dataFileIdentifier);
        }

        public void RegisterDataFolder(IDataFolder dataFolder)
        {
            dataFolderRegistry.Register(dataFolder.DataFolderIdentifier, dataFolder);
            RegisterData(dataFolder);
        }

        public IDataFolder GetDataFolder(IDataFolderIdentifier dataFolderIdentifier)
        {
            return (IDataFolder)dataFolderRegistry.Get(dataFolderIdentifier);
        }
        #endregion

        #region Overrides
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            #region Namespace/Type/Instance Registration
            TypeManager typeManager = TypeManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            IUnityInstanceType dataManagerType = (IUnityInstanceType)typeManager.GetType("LooCast.System.Management:DataManager");
            
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

            IType iDataIdentifierType = typeManager.GetType("LooCast.System.Identification:IDataIdentifier");
            IType iDataIdentifiableType = typeManager.GetType("LooCast.System.Identification:IDataIdentifiable");
            IType iDataObjectIdentifierType = typeManager.GetType("LooCast.System.Identification:IDataObjectIdentifier");
            IType iDataObjectIdentifiableType = typeManager.GetType("LooCast.System.Identification:IDataObjectIdentifiable");
            IType iDataFileIdentifierType = typeManager.GetType("LooCast.System.Identification:IDataFileIdentifier");
            IType iDataFileIdentifiableType = typeManager.GetType("LooCast.System.Identification:IDataFileIdentifiable");
            IType iDataFolderIdentifierType = typeManager.GetType("LooCast.System.Identification:IDataFolderIdentifier");
            IType iDataFolderIdentifiableType = typeManager.GetType("LooCast.System.Identification:IDataFolderIdentifiable");
            
            dataRegistry = new DataRegistry(iDataIdentifierType, iDataIdentifiableType);
            dataObjectRegistry = new DataObjectRegistry(iDataObjectIdentifierType, iDataObjectIdentifiableType);
            dataFileRegistry = new DataFileRegistry(iDataFileIdentifierType, iDataFileIdentifiableType);
            dataFolderRegistry = new DataFolderRegistry(iDataFolderIdentifierType, iDataFolderIdentifiableType);

            registryManager.RegisterRegistry(dataRegistry);
            registryManager.RegisterRegistry(dataObjectRegistry);
            registryManager.RegisterRegistry(dataFileRegistry);
            registryManager.RegisterRegistry(dataFolderRegistry);
            #endregion
        }
        #endregion
    }
}