using UnityEngine;

namespace LooCast
{
    using global::LooCast.System;
    using global::LooCast.System.Managers;
    using global::LooCast.Steamworks;
    
    public class LooCast : MonoBehaviour
    {
        #region Static Properties
        public static LooCast Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[LooCast]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    return instanceObject.AddComponent<LooCast>();
                }
                else
                {
                    return instance;
                }
            }
        }
        
        /// <summary>
        /// All InternalManagers, ordered by their Dependencies(index 0 is RegistryManager, 1 is NamespaceManager, 2 is TypeManager, 3 is InstanceManager, etc.).
        /// </summary>
        public static InternalManager[] InternalManagers
        {
            get
            {
                return new InternalManager[]
                {
                    MainManager.Instance,
                    RegistryManager.Instance,
                    NamespaceManager.Instance,
                    TypeManager.Instance,
                    SystemObjectManager.Instance,
                    GameObjectManager.Instance
                };
            }
        }

        #region Initialization Phase Flags
        public static bool IsEarlyPreInitializing { get; private set; }
        public static bool IsPreInitializing { get; private set; }
        public static bool IsLatePreInitializing { get; private set; }
        public static bool IsEarlyPreInitialized { get; private set; }
        public static bool IsPreInitialized { get; private set; }
        public static bool IsLatePreInitialized { get; private set; }

        public static bool IsEarlyInitializing { get; private set; }
        public static bool IsInitializing { get; private set; }
        public static bool IsLateInitializing { get; private set; }
        public static bool IsEarlyInitialized { get; private set; }
        public static bool IsInitialized { get; private set; }
        public static bool IsLateInitialized { get; private set; }

        public static bool IsEarlyPostInitializing { get; private set; }
        public static bool IsPostInitializing { get; private set; }
        public static bool IsLatePostInitializing { get; private set; }
        public static bool IsEarlyPostInitialized { get; private set; }
        public static bool IsPostInitialized { get; private set; }
        public static bool IsLatePostInitialized { get; private set; }

        public static bool IsFullyPreInitialized
        {
            get
            {
                return IsEarlyPreInitialized && IsPreInitialized && IsLatePreInitialized;
            }
        }
        public static bool IsFullyInitialized
        {
            get
            {
                return IsEarlyInitialized && IsInitialized && IsLateInitialized;
            }
        }
        public static bool IsFullyPostInitialized
        {
            get
            {
                return IsEarlyPostInitialized && IsPostInitialized && IsLatePostInitialized;
            }
        }
        public static bool IsCompletelyInitialized
        {
            get
            {
                return IsFullyPreInitialized && IsFullyInitialized && IsPostInitialized;
            }
        }
        #endregion

        #region Termination Phase Flags
        public static bool IsEarlyPreTerminating { get; private set; }
        public static bool IsPreTerminating { get; private set; }
        public static bool IsLatePreTerminating { get; private set; }
        public static bool IsEarlyPreTerminated { get; private set; }
        public static bool IsPreTerminated { get; private set; }
        public static bool IsLatePreTerminated { get; private set; }

        public static bool IsEarlyTerminating { get; private set; }
        public static bool IsTerminating { get; private set; }
        public static bool IsLateTerminating { get; private set; }
        public static bool IsEarlyTerminated { get; private set; }
        public static bool IsTerminated { get; private set; }
        public static bool IsLateTerminated { get; private set; }

        public static bool IsEarlyPostTerminating { get; private set; }
        public static bool IsPostTerminating { get; private set; }
        public static bool IsLatePostTerminating { get; private set; }
        public static bool IsEarlyPostTerminated { get; private set; }
        public static bool IsPostTerminated { get; private set; }
        public static bool IsLatePostTerminated { get; private set; }

        public static bool IsFullyPreTerminated
        {
            get
            {
                return IsEarlyPreTerminated && IsPreTerminated && IsLatePreTerminated;
            }
        }
        public static bool IsFullyTerminated
        {
            get
            {
                return IsEarlyTerminated && IsTerminated && IsLateTerminated;
            }
        }
        public static bool IsFullyPostTerminated
        {
            get
            {
                return IsEarlyPostTerminated && IsPostTerminated && IsLatePostTerminated;
            }
        }
        public static bool IsCompletelyTerminated
        {
            get
            {
                return IsFullyPreTerminated && IsFullyTerminated && IsPostTerminated;
            }
        }
        #endregion
        #endregion

        #region Static Fields
        private static LooCast instance;
        #endregion

        #region Unity Callbacks

        #region Initialization
        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void OnEarlyPreInitialize()
        {
            Instance.EarlyPreInitialize();
        }

        private void Awake()
        {
            EarlyInitialize();
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        private static void OnEarlyPostInitialize()
        {
            Instance.EarlyPostInitialize();
        }
        #endregion

        #region Termination
        private void OnDisable()
        {
            EarlyPreTerminate();
        }

        private void OnDestroy()
        {
            EarlyPreTerminate();
        }

        private void OnApplicationQuit()
        {
            EarlyPreTerminate();
        }
        #endregion

        #endregion

        #region Methods

        #region Initialization Phases
        private void EarlyPreInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;

            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            SystemObjectManager systemObjectManager = SystemObjectManager.Instance;
            GameObjectManager gameObjectManager = GameObjectManager.Instance;

            // TODO: Properly register namespaces, types, and instances.

            /*
            #region Loo Cast Registration
            Namespace looCastNamespace = new Namespace("LooCast");
            namespaceManager.RegisterNamespace(looCastNamespace);

            GameObjectType mainManagerType = new GameObjectType(typeof(MainManager), looCastNamespace);
            typeManager.RegisterType(mainManagerType);

            GameObject mainManagerInstance = new GameObject(this, mainManagerType);
            gameObjectManager.RegisterGameObject(mainManagerInstance);

            #region System
            Namespace looCastSystemNamespace = new Namespace("System", looCastNamespace);
            namespaceManager.RegisterNamespace(looCastSystemNamespace);

            SystemObjectType iPersistableType = new SystemObjectType(typeof(IPersistable), looCastSystemNamespace);
            SystemObjectType iNamespaceType = new SystemObjectType(typeof(INamespace), looCastSystemNamespace);
            SystemObjectType iInstanceType = new SystemObjectType(typeof(IInstance), looCastSystemNamespace);
            SystemObjectType iSystemObjectType = new SystemObjectType(typeof(ISystemObject), looCastSystemNamespace);
            SystemObjectType iGameObjectType = new SystemObjectType(typeof(IGameObject), looCastSystemNamespace);
            SystemObjectType iObjectType = new SystemObjectType(typeof(IObject), looCastSystemNamespace);
            SystemObjectType iGameObjectType = new SystemObjectType(typeof(IGameObject), looCastSystemNamespace);
            SystemObjectType iComponentType = new SystemObjectType(typeof(IComponent), looCastSystemNamespace);
            SystemObjectType namespaceType = new SystemObjectType(typeof(Namespace), looCastSystemNamespace);
            SystemObjectType systemObjectType = new SystemObjectType(typeof(SystemObject), looCastSystemNamespace);
            SystemObjectType gameObjectType = new SystemObjectType(typeof(GameObject), looCastSystemNamespace);
            SystemObjectType extendedMonoBehaviourType = new SystemObjectType(typeof(ExtendedMonoBehaviour), looCastSystemNamespace);
            SystemObjectType bigFloatType = new SystemObjectType(typeof(BigFloat), looCastSystemNamespace);
            SystemObjectType bigVector3Type = new SystemObjectType(typeof(BigVector3), looCastSystemNamespace);
            typeManager.RegisterType(iPersistableType);
            typeManager.RegisterType(iNamespaceType);
            typeManager.RegisterType(iInstanceType);
            typeManager.RegisterType(iSystemObjectType);
            typeManager.RegisterType(iGameObjectType);
            typeManager.RegisterType(iObjectType);
            typeManager.RegisterType(iGameObjectType);
            typeManager.RegisterType(iComponentType);
            typeManager.RegisterType(namespaceType);
            typeManager.RegisterType(systemObjectType);
            typeManager.RegisterType(gameObjectType);
            typeManager.RegisterType(extendedMonoBehaviourType);
            typeManager.RegisterType(bigFloatType);
            typeManager.RegisterType(bigVector3Type);

            #region Data
            Namespace looCastSystemDataNamespace = new Namespace("Data", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemDataNamespace);
            #endregion

            #region Exceptions
            Namespace looCastSystemExceptionsNamespace = new Namespace("Exceptions", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemExceptionsNamespace);
            #endregion

            #region Identification
            Namespace looCastSystemIdentificationNamespace = new Namespace("Identification", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemIdentificationNamespace);

            SystemObjectType iIdentifierType = new SystemObjectType(typeof(IIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iIdentifiableType = new SystemObjectType(typeof(IIdentifiable), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iIdentifierType);
            typeManager.RegisterType(iIdentifiableType);

            SystemObjectType iRegistryIdentifierType = new SystemObjectType(typeof(IRegistryIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iRegistryIdentifiableType = new SystemObjectType(typeof(IRegistryIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType registryIdentifierType = new SystemObjectType(typeof(RegistryIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iRegistryIdentifierType);
            typeManager.RegisterType(iRegistryIdentifiableType);
            typeManager.RegisterType(registryIdentifierType);

            SystemObjectType iNamespaceIdentifierType = new SystemObjectType(typeof(INamespaceIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iNamespaceIdentifiableType = new SystemObjectType(typeof(INamespaceIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType namespaceIdentifierType = new SystemObjectType(typeof(NamespaceIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iNamespaceIdentifierType);
            typeManager.RegisterType(iNamespaceIdentifiableType);
            typeManager.RegisterType(namespaceIdentifierType);

            SystemObjectType iTypeIdentifierType = new SystemObjectType(typeof(ITypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iTypeIdentifiableType = new SystemObjectType(typeof(ITypeIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType typeIdentifierType = new SystemObjectType(typeof(TypeIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iTypeIdentifierType);
            typeManager.RegisterType(iTypeIdentifiableType);
            typeManager.RegisterType(typeIdentifierType);

            SystemObjectType iMetaDataIdentifier = new SystemObjectType(typeof(IMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iMetaDataIdentifiable = new SystemObjectType(typeof(IMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iMetaDataTypeIdentifier = new SystemObjectType(typeof(IMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType metaDataIdentifier = new SystemObjectType(typeof(MetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iMetaDataIdentifier);
            typeManager.RegisterType(iMetaDataIdentifiable);
            typeManager.RegisterType(iMetaDataTypeIdentifier);
            typeManager.RegisterType(metaDataIdentifier);

            SystemObjectType iDataIdentifier = new SystemObjectType(typeof(IDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iDataIdentifiable = new SystemObjectType(typeof(IDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iDataTypeIdentifier = new SystemObjectType(typeof(IDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType dataIdentifier = new SystemObjectType(typeof(DataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iDataIdentifier);
            typeManager.RegisterType(iDataIdentifiable);
            typeManager.RegisterType(iDataTypeIdentifier);
            typeManager.RegisterType(dataIdentifier);

            SystemObjectType iDataObjectIdentifier = new SystemObjectType(typeof(IDataObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iDataObjectIdentifiable = new SystemObjectType(typeof(IDataObjectIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iDataObjectTypeIdentifier = new SystemObjectType(typeof(IDataObjectTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iDataObjectMetaDataIdentifier = new SystemObjectType(typeof(IDataObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iDataObjectMetaDataIdentifiable = new SystemObjectType(typeof(IDataObjectMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iDataObjectMetaDataTypeIdentifier = new SystemObjectType(typeof(IDataObjectMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType dataObjectIdentifier = new SystemObjectType(typeof(DataObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType dataObjectMetaDataIdentifier = new SystemObjectType(typeof(DataObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iDataObjectIdentifier);
            typeManager.RegisterType(iDataObjectIdentifiable);
            typeManager.RegisterType(iDataObjectTypeIdentifier);
            typeManager.RegisterType(iDataObjectMetaDataIdentifier);
            typeManager.RegisterType(iDataObjectMetaDataIdentifiable);
            typeManager.RegisterType(iDataObjectMetaDataTypeIdentifier);
            typeManager.RegisterType(dataObjectIdentifier);
            typeManager.RegisterType(dataObjectMetaDataIdentifier);

            SystemObjectType iDataFileIdentifier = new SystemObjectType(typeof(IDataFileIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iDataFileIdentifiable = new SystemObjectType(typeof(IDataFileIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iDataFileTypeIdentifier = new SystemObjectType(typeof(IDataFileTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iDataFileMetaDataIdentifier = new SystemObjectType(typeof(IDataFileMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iDataFileMetaDataIdentifiable = new SystemObjectType(typeof(IDataFileMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iDataFileMetaDataTypeIdentifier = new SystemObjectType(typeof(IDataFileMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType dataFileIdentifier = new SystemObjectType(typeof(DataFileIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType dataFileMetaDataIdentifier = new SystemObjectType(typeof(DataFileMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iDataFileIdentifier);
            typeManager.RegisterType(iDataFileIdentifiable);
            typeManager.RegisterType(iDataFileTypeIdentifier);
            typeManager.RegisterType(iDataFileMetaDataIdentifier);
            typeManager.RegisterType(iDataFileMetaDataIdentifiable);
            typeManager.RegisterType(iDataFileMetaDataTypeIdentifier);
            typeManager.RegisterType(dataFileIdentifier);
            typeManager.RegisterType(dataFileMetaDataIdentifier);

            SystemObjectType iDataFolderIdentifier = new SystemObjectType(typeof(IDataFolderIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iDataFolderIdentifiable = new SystemObjectType(typeof(IDataFolderIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iDataFolderTypeIdentifier = new SystemObjectType(typeof(IDataFolderTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iDataFolderMetaDataIdentifier = new SystemObjectType(typeof(IDataFolderMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iDataFolderMetaDataIdentifiable = new SystemObjectType(typeof(IDataFolderMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iDataFolderMetaDataTypeIdentifier = new SystemObjectType(typeof(IDataFolderMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType dataFolderIdentifier = new SystemObjectType(typeof(DataFolderIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType dataFolderMetaDataIdentifier = new SystemObjectType(typeof(DataFolderMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iDataFolderIdentifier);
            typeManager.RegisterType(iDataFolderIdentifiable);
            typeManager.RegisterType(iDataFolderTypeIdentifier);
            typeManager.RegisterType(iDataFolderMetaDataIdentifier);
            typeManager.RegisterType(iDataFolderMetaDataIdentifiable);
            typeManager.RegisterType(iDataFolderMetaDataTypeIdentifier);
            typeManager.RegisterType(dataFolderIdentifier);
            typeManager.RegisterType(dataFolderMetaDataIdentifier);

            SystemObjectType iInstanceIdentifier = new SystemObjectType(typeof(IInstanceIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iInstanceIdentifiable = new SystemObjectType(typeof(IInstanceIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iInstanceTypeIdentifier = new SystemObjectType(typeof(IInstanceTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iInstanceDataIdentifier = new SystemObjectType(typeof(IInstanceDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iInstanceDataIdentifiable = new SystemObjectType(typeof(IInstanceDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iInstanceDataTypeIdentifier = new SystemObjectType(typeof(IInstanceDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iInstanceMetaDataIdentifier = new SystemObjectType(typeof(IInstanceMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iInstanceMetaDataIdentifiable = new SystemObjectType(typeof(IInstanceMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iInstanceMetaDataTypeIdentifier = new SystemObjectType(typeof(IInstanceMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType instanceIdentifier = new SystemObjectType(typeof(InstanceIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType instanceDataIdentifier = new SystemObjectType(typeof(InstanceDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType instanceMetaDataIdentifier = new SystemObjectType(typeof(InstanceMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iInstanceIdentifier);
            typeManager.RegisterType(iInstanceIdentifiable);
            typeManager.RegisterType(iInstanceTypeIdentifier);
            typeManager.RegisterType(iInstanceDataIdentifier);
            typeManager.RegisterType(iInstanceDataIdentifiable);
            typeManager.RegisterType(iInstanceDataTypeIdentifier);
            typeManager.RegisterType(iInstanceMetaDataIdentifier);
            typeManager.RegisterType(iInstanceMetaDataIdentifiable);
            typeManager.RegisterType(iInstanceMetaDataTypeIdentifier);
            typeManager.RegisterType(instanceIdentifier);
            typeManager.RegisterType(instanceDataIdentifier);
            typeManager.RegisterType(instanceMetaDataIdentifier);

            SystemObjectType iSystemObjectIdentifier = new SystemObjectType(typeof(ISystemObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iSystemObjectIdentifiable = new SystemObjectType(typeof(ISystemObjectIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iSystemObjectTypeIdentifier = new SystemObjectType(typeof(ISystemObjectTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iSystemObjectDataIdentifier = new SystemObjectType(typeof(ISystemObjectDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iSystemObjectDataIdentifiable = new SystemObjectType(typeof(ISystemObjectDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iSystemObjectDataTypeIdentifier = new SystemObjectType(typeof(ISystemObjectDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iSystemObjectMetaDataIdentifier = new SystemObjectType(typeof(ISystemObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iSystemObjectMetaDataIdentifiable = new SystemObjectType(typeof(ISystemObjectMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iSystemObjectMetaDataTypeIdentifier = new SystemObjectType(typeof(ISystemObjectMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType systemObjectIdentifier = new SystemObjectType(typeof(SystemObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType systemObjectDataIdentifier = new SystemObjectType(typeof(SystemObjectDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType systemObjectMetaDataIdentifier = new SystemObjectType(typeof(SystemObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iSystemObjectIdentifier);
            typeManager.RegisterType(iSystemObjectIdentifiable);
            typeManager.RegisterType(iSystemObjectTypeIdentifier);
            typeManager.RegisterType(iSystemObjectDataIdentifier);
            typeManager.RegisterType(iSystemObjectDataIdentifiable);
            typeManager.RegisterType(iSystemObjectDataTypeIdentifier);
            typeManager.RegisterType(iSystemObjectMetaDataIdentifier);
            typeManager.RegisterType(iSystemObjectMetaDataIdentifiable);
            typeManager.RegisterType(iSystemObjectMetaDataTypeIdentifier);
            typeManager.RegisterType(systemObjectIdentifier);
            typeManager.RegisterType(systemObjectDataIdentifier);
            typeManager.RegisterType(systemObjectMetaDataIdentifier);

            SystemObjectType iGameObjectIdentifier = new SystemObjectType(typeof(IGameObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectIdentifiable = new SystemObjectType(typeof(IGameObjectIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectTypeIdentifier = new SystemObjectType(typeof(IGameObjectTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectDataIdentifier = new SystemObjectType(typeof(IGameObjectDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectDataIdentifiable = new SystemObjectType(typeof(IGameObjectDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectDataTypeIdentifier = new SystemObjectType(typeof(IGameObjectDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectMetaDataIdentifier = new SystemObjectType(typeof(IGameObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectMetaDataIdentifiable = new SystemObjectType(typeof(IGameObjectMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectMetaDataTypeIdentifier = new SystemObjectType(typeof(IGameObjectMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType gameObjectIdentifier = new SystemObjectType(typeof(GameObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType gameObjectDataIdentifier = new SystemObjectType(typeof(GameObjectDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType gameObjectMetaDataIdentifier = new SystemObjectType(typeof(GameObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iGameObjectIdentifier);
            typeManager.RegisterType(iGameObjectIdentifiable);
            typeManager.RegisterType(iGameObjectTypeIdentifier);
            typeManager.RegisterType(iGameObjectDataIdentifier);
            typeManager.RegisterType(iGameObjectDataIdentifiable);
            typeManager.RegisterType(iGameObjectDataTypeIdentifier);
            typeManager.RegisterType(iGameObjectMetaDataIdentifier);
            typeManager.RegisterType(iGameObjectMetaDataIdentifiable);
            typeManager.RegisterType(iGameObjectMetaDataTypeIdentifier);
            typeManager.RegisterType(gameObjectIdentifier);
            typeManager.RegisterType(gameObjectDataIdentifier);
            typeManager.RegisterType(gameObjectMetaDataIdentifier);

            SystemObjectType iObjectIdentifier = new SystemObjectType(typeof(IObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iObjectIdentifiable = new SystemObjectType(typeof(IObjectIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iObjectTypeIdentifier = new SystemObjectType(typeof(IObjectTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iObjectDataIdentifier = new SystemObjectType(typeof(IObjectDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iObjectDataIdentifiable = new SystemObjectType(typeof(IObjectDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iObjectDataTypeIdentifier = new SystemObjectType(typeof(IObjectDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iObjectMetaDataIdentifier = new SystemObjectType(typeof(IObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iObjectMetaDataIdentifiable = new SystemObjectType(typeof(IObjectMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iObjectMetaDataTypeIdentifier = new SystemObjectType(typeof(IObjectMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType objectIdentifier = new SystemObjectType(typeof(ObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType objectDataIdentifier = new SystemObjectType(typeof(ObjectDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType objectMetaDataIdentifier = new SystemObjectType(typeof(ObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iObjectIdentifier);
            typeManager.RegisterType(iObjectIdentifiable);
            typeManager.RegisterType(iObjectTypeIdentifier);
            typeManager.RegisterType(iObjectDataIdentifier);
            typeManager.RegisterType(iObjectDataIdentifiable);
            typeManager.RegisterType(iObjectDataTypeIdentifier);
            typeManager.RegisterType(iObjectMetaDataIdentifier);
            typeManager.RegisterType(iObjectMetaDataIdentifiable);
            typeManager.RegisterType(iObjectMetaDataTypeIdentifier);
            typeManager.RegisterType(objectIdentifier);
            typeManager.RegisterType(objectDataIdentifier);
            typeManager.RegisterType(objectMetaDataIdentifier);

            SystemObjectType iGameObjectIdentifier = new SystemObjectType(typeof(IGameObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectIdentifiable = new SystemObjectType(typeof(IGameObjectIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectTypeIdentifier = new SystemObjectType(typeof(IGameObjectTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectDataIdentifier = new SystemObjectType(typeof(IGameObjectDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectDataIdentifiable = new SystemObjectType(typeof(IGameObjectDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectDataTypeIdentifier = new SystemObjectType(typeof(IGameObjectDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectMetaDataIdentifier = new SystemObjectType(typeof(IGameObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectMetaDataIdentifiable = new SystemObjectType(typeof(IGameObjectMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iGameObjectMetaDataTypeIdentifier = new SystemObjectType(typeof(IGameObjectMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType gameObjectIdentifier = new SystemObjectType(typeof(GameObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType gameObjectDataIdentifier = new SystemObjectType(typeof(GameObjectDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType gameObjectMetaDataIdentifier = new SystemObjectType(typeof(GameObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iGameObjectIdentifier);
            typeManager.RegisterType(iGameObjectIdentifiable);
            typeManager.RegisterType(iGameObjectTypeIdentifier);
            typeManager.RegisterType(iGameObjectDataIdentifier);
            typeManager.RegisterType(iGameObjectDataIdentifiable);
            typeManager.RegisterType(iGameObjectDataTypeIdentifier);
            typeManager.RegisterType(iGameObjectMetaDataIdentifier);
            typeManager.RegisterType(iGameObjectMetaDataIdentifiable);
            typeManager.RegisterType(iGameObjectMetaDataTypeIdentifier);
            typeManager.RegisterType(gameObjectIdentifier);
            typeManager.RegisterType(gameObjectDataIdentifier);
            typeManager.RegisterType(gameObjectMetaDataIdentifier);

            SystemObjectType iComponentIdentifier = new SystemObjectType(typeof(IComponentIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iComponentIdentifiable = new SystemObjectType(typeof(IComponentIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iComponentTypeIdentifier = new SystemObjectType(typeof(IComponentTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iComponentDataIdentifier = new SystemObjectType(typeof(IComponentDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iComponentDataIdentifiable = new SystemObjectType(typeof(IComponentDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iComponentDataTypeIdentifier = new SystemObjectType(typeof(IComponentDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iComponentMetaDataIdentifier = new SystemObjectType(typeof(IComponentMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iComponentMetaDataIdentifiable = new SystemObjectType(typeof(IComponentMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iComponentMetaDataTypeIdentifier = new SystemObjectType(typeof(IComponentMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType componentIdentifier = new SystemObjectType(typeof(ComponentIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType componentDataIdentifier = new SystemObjectType(typeof(ComponentDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType componentMetaDataIdentifier = new SystemObjectType(typeof(ComponentMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iComponentIdentifier);
            typeManager.RegisterType(iComponentIdentifiable);
            typeManager.RegisterType(iComponentTypeIdentifier);
            typeManager.RegisterType(iComponentDataIdentifier);
            typeManager.RegisterType(iComponentDataIdentifiable);
            typeManager.RegisterType(iComponentDataTypeIdentifier);
            typeManager.RegisterType(iComponentMetaDataIdentifier);
            typeManager.RegisterType(iComponentMetaDataIdentifiable);
            typeManager.RegisterType(iComponentMetaDataTypeIdentifier);
            typeManager.RegisterType(componentIdentifier);
            typeManager.RegisterType(componentDataIdentifier);
            typeManager.RegisterType(componentMetaDataIdentifier);

            SystemObjectType iResourceIdentifier = new SystemObjectType(typeof(IResourceIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceIdentifiable = new SystemObjectType(typeof(IResourceIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceTypeIdentifier = new SystemObjectType(typeof(IResourceTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceDataIdentifier = new SystemObjectType(typeof(IResourceDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceDataIdentifiable = new SystemObjectType(typeof(IResourceDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceDataTypeIdentifier = new SystemObjectType(typeof(IResourceDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceMetaDataIdentifier = new SystemObjectType(typeof(IResourceMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceMetaDataIdentifiable = new SystemObjectType(typeof(IResourceMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceMetaDataTypeIdentifier = new SystemObjectType(typeof(IResourceMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceIdentifier = new SystemObjectType(typeof(ResourceIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceDataIdentifier = new SystemObjectType(typeof(ResourceDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceMetaDataIdentifier = new SystemObjectType(typeof(ResourceMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iResourceIdentifier);
            typeManager.RegisterType(iResourceIdentifiable);
            typeManager.RegisterType(iResourceTypeIdentifier);
            typeManager.RegisterType(iResourceDataIdentifier);
            typeManager.RegisterType(iResourceDataIdentifiable);
            typeManager.RegisterType(iResourceDataTypeIdentifier);
            typeManager.RegisterType(iResourceMetaDataIdentifier);
            typeManager.RegisterType(iResourceMetaDataIdentifiable);
            typeManager.RegisterType(iResourceMetaDataTypeIdentifier);
            typeManager.RegisterType(resourceIdentifier);
            typeManager.RegisterType(resourceDataIdentifier);
            typeManager.RegisterType(resourceMetaDataIdentifier);

            SystemObjectType iResourceObjectIdentifier = new SystemObjectType(typeof(IResourceObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceObjectIdentifiable = new SystemObjectType(typeof(IResourceObjectIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceObjectTypeIdentifier = new SystemObjectType(typeof(IResourceObjectTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceObjectDataIdentifier = new SystemObjectType(typeof(IResourceObjectDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceObjectDataIdentifiable = new SystemObjectType(typeof(IResourceObjectDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceObjectDataTypeIdentifier = new SystemObjectType(typeof(IResourceObjectDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceObjectMetaDataIdentifier = new SystemObjectType(typeof(IResourceObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceObjectMetaDataIdentifiable = new SystemObjectType(typeof(IResourceObjectMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceObjectMetaDataTypeIdentifier = new SystemObjectType(typeof(IResourceObjectMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceObjectIdentifier = new SystemObjectType(typeof(ResourceObjectIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceObjectDataIdentifier = new SystemObjectType(typeof(ResourceObjectDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceObjectMetaDataIdentifier = new SystemObjectType(typeof(ResourceObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iResourceObjectIdentifier);
            typeManager.RegisterType(iResourceObjectIdentifiable);
            typeManager.RegisterType(iResourceObjectTypeIdentifier);
            typeManager.RegisterType(iResourceObjectDataIdentifier);
            typeManager.RegisterType(iResourceObjectDataIdentifiable);
            typeManager.RegisterType(iResourceObjectDataTypeIdentifier);
            typeManager.RegisterType(iResourceObjectMetaDataIdentifier);
            typeManager.RegisterType(iResourceObjectMetaDataIdentifiable);
            typeManager.RegisterType(iResourceObjectMetaDataTypeIdentifier);
            typeManager.RegisterType(resourceObjectIdentifier);
            typeManager.RegisterType(resourceObjectDataIdentifier);
            typeManager.RegisterType(resourceObjectMetaDataIdentifier);

            SystemObjectType iResourceFileIdentifier = new SystemObjectType(typeof(IResourceFileIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFileIdentifiable = new SystemObjectType(typeof(IResourceFileIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFileTypeIdentifier = new SystemObjectType(typeof(IResourceFileTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFileDataIdentifier = new SystemObjectType(typeof(IResourceFileDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFileDataIdentifiable = new SystemObjectType(typeof(IResourceFileDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFileDataTypeIdentifier = new SystemObjectType(typeof(IResourceFileDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFileMetaDataIdentifier = new SystemObjectType(typeof(IResourceFileMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFileMetaDataIdentifiable = new SystemObjectType(typeof(IResourceFileMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFileMetaDataTypeIdentifier = new SystemObjectType(typeof(IResourceFileMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceFileIdentifier = new SystemObjectType(typeof(ResourceFileIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceFileDataIdentifier = new SystemObjectType(typeof(ResourceFileDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceFileMetaDataIdentifier = new SystemObjectType(typeof(ResourceFileMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iResourceFileIdentifier);
            typeManager.RegisterType(iResourceFileIdentifiable);
            typeManager.RegisterType(iResourceFileTypeIdentifier);
            typeManager.RegisterType(iResourceFileDataIdentifier);
            typeManager.RegisterType(iResourceFileDataIdentifiable);
            typeManager.RegisterType(iResourceFileDataTypeIdentifier);
            typeManager.RegisterType(iResourceFileMetaDataIdentifier);
            typeManager.RegisterType(iResourceFileMetaDataIdentifiable);
            typeManager.RegisterType(iResourceFileMetaDataTypeIdentifier);
            typeManager.RegisterType(resourceFileIdentifier);
            typeManager.RegisterType(resourceFileDataIdentifier);
            typeManager.RegisterType(resourceFileMetaDataIdentifier);

            SystemObjectType iResourceFolderIdentifier = new SystemObjectType(typeof(IResourceFolderIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFolderIdentifiable = new SystemObjectType(typeof(IResourceFolderIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFolderTypeIdentifier = new SystemObjectType(typeof(IResourceFolderTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFolderDataIdentifier = new SystemObjectType(typeof(IResourceFolderDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFolderDataIdentifiable = new SystemObjectType(typeof(IResourceFolderDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFolderDataTypeIdentifier = new SystemObjectType(typeof(IResourceFolderDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFolderMetaDataIdentifier = new SystemObjectType(typeof(IResourceFolderMetaDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFolderMetaDataIdentifiable = new SystemObjectType(typeof(IResourceFolderMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            SystemObjectType iResourceFolderMetaDataTypeIdentifier = new SystemObjectType(typeof(IResourceFolderMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceFolderIdentifier = new SystemObjectType(typeof(ResourceFolderIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceFolderDataIdentifier = new SystemObjectType(typeof(ResourceFolderDataIdentifier), looCastSystemIdentificationNamespace);
            SystemObjectType resourceFolderMetaDataIdentifier = new SystemObjectType(typeof(ResourceFolderMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iResourceFolderIdentifier);
            typeManager.RegisterType(iResourceFolderIdentifiable);
            typeManager.RegisterType(iResourceFolderTypeIdentifier);
            typeManager.RegisterType(iResourceFolderDataIdentifier);
            typeManager.RegisterType(iResourceFolderDataIdentifiable);
            typeManager.RegisterType(iResourceFolderDataTypeIdentifier);
            typeManager.RegisterType(iResourceFolderMetaDataIdentifier);
            typeManager.RegisterType(iResourceFolderMetaDataIdentifiable);
            typeManager.RegisterType(iResourceFolderMetaDataTypeIdentifier);
            typeManager.RegisterType(resourceFolderIdentifier);
            typeManager.RegisterType(resourceFolderDataIdentifier);
            typeManager.RegisterType(resourceFolderMetaDataIdentifier);
            #endregion

            #region Management
            Namespace looCastSystemManagementNamespace = new Namespace("Management", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemManagementNamespace);
            #endregion

            #region MetaData
            Namespace looCastSystemMetaDataNamespace = new Namespace("MetaData", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemMetaDataNamespace);
            #endregion

            #region Registration
            Namespace looCastSystemRegistrationNamespace = new Namespace("Registration", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemRegistrationNamespace);
            #endregion

            #region Resources
            Namespace looCastSystemResourcesNamespace = new Namespace("Resources", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemResourcesNamespace);
            #endregion

            #region Types
            Namespace looCastSystemTypesNamespace = new Namespace("Types", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemTypesNamespace);
            #endregion

            #endregion

            #endregion
            */

            #region Internal Managers Setup

            #region Pre-Initialization
            Debug.Log($"[LooCast] Pre-Initializing internal module manager instances.");
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.PreInitializeInstance();
            }
            Debug.Log($"[LooCast] Pre-Initialized internal module manager instances.");
            #endregion

            #region Initialization
            Debug.Log($"[LooCast] Initializing internal module manager instances.");
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.InitializeInstance();
            }
            Debug.Log($"[LooCast] Initialized internal module manager instances.");
            #endregion

            #region Post-Initialization
            Debug.Log($"[LooCast] Post-Initializing internal module manager instances.");
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.PostInitializeInstance();
            }
            Debug.Log($"[LooCast] Post-Initialized internal module manager instances.");
            #endregion

            #endregion

            IsEarlyPreInitializing = true;
            Debug.Log($"[LooCast] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            #region Early Pre-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.EarlyPreInitialize();
            }
            #endregion

            #endregion

            IsEarlyPreInitializing = false;
            IsEarlyPreInitialized = true;
            Debug.Log($"[LooCast] Finished Early Pre-Initialization in Scene '{activeSceneName}'.");

            PreInitialize();
        }

        private void PreInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPreInitializing = true;
            Debug.Log($"[LooCast] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            #region Pre-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.PreInitialize();
            }
            #endregion

            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;
            Debug.Log($"[LooCast] Finished Pre-Initialization in Scene '{activeSceneName}'.");

            LatePreInitialize();
        }

        private void LatePreInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePreInitializing = true;
            Debug.Log($"[LooCast] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            #region Late Pre-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.LatePreInitialize();
            }
            #endregion

            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;
            Debug.Log($"[LooCast] Finished Pre-Initialization in Scene '{activeSceneName}'.");

            _ = Instance;
        }

        private void EarlyInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyInitializing = true;
            Debug.Log($"[LooCast] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            #region Early Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.EarlyInitialize();
            }
            #endregion

            #endregion

            IsEarlyInitializing = false;
            IsEarlyInitialized = true;
            Debug.Log($"[LooCast] Finished Early Pre-Initialization in Scene '{activeSceneName}'.");

            Initialize();
        }

        private void Initialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsInitializing = true;
            Debug.Log($"[LooCast] Starting Initialization in Scene '{activeSceneName}'.");

            #region Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.Initialize();
            }
            #endregion
            
            // TODO: SteamManager, Utilities and Scene should not be initialized here!
            #region SteamManager
            _ = SteamworksManager.Initialized;
            #endregion

            #region Utilities
            Universe.Universe.DensityMapGenerationUtil.InitializeInstance();
            #endregion

            #region Scene
            switch (activeSceneName)
            {
                case "MainMenu":
                    break;
                case "Game":
                    SceneManager.Instance.AddPostSceneLoadAction(() =>
                    {
                        GameManager gameManager = FindObjectOfType<GameManager>();
                        if (gameManager.Games.Contains("New Game"))
                        {
                            gameManager.InitializeGame(gameManager.Games.GetGame("New Game"));
                        }
                        else
                        {
                            gameManager.InitializeGame("New Game");
                        }
                    });
                    break;
            }
            #endregion

            #endregion

            IsInitializing = false;
            IsInitialized = true;
            Debug.Log($"[LooCast] Finished Initialization in Scene '{activeSceneName}'.");

            LateInitialize();
        }

        private void LateInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLateInitializing = true;
            Debug.Log($"[LooCast] Starting Late Pre-Initialization in Scene '{activeSceneName}'.");

            #region Late Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.LateInitialize();
            }
            #endregion

            #endregion

            IsLateInitializing = false;
            IsLateInitialized = true;
            Debug.Log($"[LooCast] Finished Late Pre-Initialization in Scene '{activeSceneName}'.");
        }

        private void EarlyPostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPostInitializing = true;
            Debug.Log($"[LooCast] Starting Early Post-Initialization in Scene '{activeSceneName}'.");

            #region Early Post-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.EarlyPostInitalize();
            }
            #endregion

            #endregion

            IsEarlyPostInitializing = false;
            IsEarlyPostInitialized = true;
            Debug.Log($"[LooCast] Finished Early Post-Initialization in Scene '{activeSceneName}'.");

            PostInitialize();
        }

        private void PostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPostInitializing = true;
            Debug.Log($"[LooCast] Starting Post-Initialization in Scene '{activeSceneName}'.");

            #region Post-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.PostInitialize();
            }
            #endregion

            #endregion

            IsPostInitializing = false;
            IsPostInitialized = true;
            Debug.Log($"[LooCast] Finished Post-Initialization in Scene '{activeSceneName}'.");

            LatePostInitialize();
        }

        private void LatePostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePostInitializing = true;
            Debug.Log($"[LooCast] Starting Late Post-Initialization in Scene '{activeSceneName}'.");

            #region Late Post-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.LatePostInitialize();
            }
            #endregion

            #endregion

            IsLatePostInitializing = false;
            IsLatePostInitialized = true;
            Debug.Log($"[LooCast] Finished Late Post-Initialization in Scene '{activeSceneName}'.");
        }
        #endregion

        #region Termination Phases
        private void EarlyPreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPreTerminating = true;
            Debug.Log($"[LooCast] Starting Early Pre-Termination in Scene '{activeSceneName}'.");

            #region Early Pre-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers.Reverse())
            {
                internalManager.EarlyPreTerminate();
            }
            #endregion

            #endregion

            IsEarlyPreTerminating = false;
            IsEarlyPreTerminated = true;
            Debug.Log($"[LooCast] Finished Early Pre-Termination in Scene '{activeSceneName}'.");

            PreTerminate();
        }

        private void PreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPreTerminating = true;
            Debug.Log($"[LooCast] Starting Pre-Termination in Scene '{activeSceneName}'.");

            #region Pre-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers.Reverse())
            {
                internalManager.PreTerminate();
            }
            #endregion

            #endregion

            IsPreTerminating = false;
            IsPreTerminated = true;
            Debug.Log($"[LooCast] Finished Pre-Termination in Scene '{activeSceneName}'.");

            LatePreTerminate();
        }

        private void LatePreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePreTerminating = true;
            Debug.Log($"[LooCast] Starting Late Pre-Termination in Scene '{activeSceneName}'.");

            #region Late Pre-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers.Reverse())
            {
                internalManager.LatePreTerminate();
            }
            #endregion

            #endregion

            IsLatePreTerminating = false;
            IsLatePreTerminated = true;
            Debug.Log($"[LooCast] Finished Late Pre-Termination in Scene '{activeSceneName}'.");

            EarlyTerminate();
        }

        private void EarlyTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyTerminating = true;
            Debug.Log($"[LooCast] Starting Early Termination in Scene '{activeSceneName}'.");

            #region Early Termination

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers.Reverse())
            {
                internalManager.EarlyTerminate();
            }
            #endregion

            #endregion

            IsEarlyTerminating = false;
            IsEarlyTerminated = true;
            Debug.Log($"[LooCast] Finished Early Termination in Scene '{activeSceneName}'.");

            Terminate();
        }

        private void Terminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsTerminating = true;
            Debug.Log($"[LooCast] Starting Termination in Scene '{activeSceneName}'.");

            #region Termination

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers.Reverse())
            {
                internalManager.Terminate();
            }
            #endregion

            #endregion

            IsTerminating = false;
            IsTerminated = true;
            Debug.Log($"[LooCast] Finished Termination in Scene '{activeSceneName}'.");

            LateTerminate();
        }

        private void LateTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLateTerminating = true;
            Debug.Log($"[LooCast] Starting Late Termination in Scene '{activeSceneName}'.");

            #region Late Termination

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers.Reverse())
            {
                internalManager.LateTerminate();
            }
            #endregion

            #endregion

            IsLateTerminating = false;
            IsLateTerminated = true;
            Debug.Log($"[LooCast] Finished Late Termination in Scene '{activeSceneName}'.");

            EarlyPostTerminate();
        }

        private void EarlyPostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPostTerminating = true;
            Debug.Log($"[LooCast] Starting Early Post-Termination in Scene '{activeSceneName}'.");

            #region Early Post-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers.Reverse())
            {
                internalManager.EarlyPostTerminate();
            }
            #endregion

            #endregion

            IsEarlyPostTerminating = false;
            IsEarlyPostTerminated = true;
            Debug.Log($"[LooCast] Finished Early Post-Termination in Scene '{activeSceneName}'.");

            PostTerminate();
        }

        private void PostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPostTerminating = true;
            Debug.Log($"[LooCast] Starting Post-Termination in Scene '{activeSceneName}'.");

            #region Post-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers.Reverse())
            {
                internalManager.PostTerminate();
            }
            #endregion

            #endregion

            IsPostTerminating = false;
            IsPostTerminated = true;
            Debug.Log($"[LooCast] Finished Post-Termination in Scene '{activeSceneName}'.");

            LatePostTerminate();
        }

        private void LatePostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePostTerminating = true;
            Debug.Log($"[LooCast] Starting Late Post-Termination in Scene '{activeSceneName}'.");

            #region Late Post-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in InternalManagers.Reverse())
            {
                internalManager.LatePostTerminate();
            }
            #endregion

            #endregion

            IsLatePostTerminating = false;
            IsLatePostTerminated = true;
            Debug.Log($"[LooCast] Finished Late Post-Termination in Scene '{activeSceneName}'.");
        }
        #endregion

        #endregion
    }
}
