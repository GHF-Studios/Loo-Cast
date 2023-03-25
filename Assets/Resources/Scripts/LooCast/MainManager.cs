using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast
{
    using LooCast.System;
    using LooCast.System.Exceptions;
    using LooCast.System.Identification;
    using LooCast.System.Management;
    using LooCast.System.Registration;
    using LooCast.System.Types;
    using LooCast.Game;
    using LooCast.Scene;
    using LooCast.Steamworks;
    using LooCast.Util;
    using UnityEditor.Build.Content;

    public class MainManager : MonoBehaviour, IUnityInstanceIdentifiable
    {
        #region Static Properties
        
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

        public static MainManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[MainManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    return instanceObject.AddComponent<MainManager>();
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
                    RegistryManager.Instance,
                    NamespaceManager.Instance,
                    TypeManager.Instance,
                    CSharpInstanceManager.Instance,
                    UnityInstanceManager.Instance
                };
            }
        }
        /// <summary>
        /// All CoreModuleManagers, ordered by their Dependencies(index 0 is Base Mod Core Module Manager, 1 is Mod Core Module Manager, 2 is Mod Extension Core Module Manager, 3 is Mod Extension Extension Core Module Manager, etc.).
        /// </summary>
        public static CoreModuleManager[] CoreModuleManagers
        {
            get
            {
                // TODO: Implement loading/deserialization/injection of CoreModuleManagers.
            }
        }
        #endregion

        #region Static Fields
        private static MainManager instance;
        #endregion

        #region Properties
        public IIdentifier Identifier => unityInstanceIdentifier;
        public IInstanceIdentifier InstanceIdentifier => unityInstanceIdentifier;
        public IUnityInstanceIdentifier UnityInstanceIdentifier => unityInstanceIdentifier;
        
        public string RootPersistentPath
        {
            get
            {
                if (string.IsNullOrEmpty(rootPersistentPath))
                {
                    rootPersistentPath = Application.persistentDataPath;
                }
                return rootPersistentPath;
            }
        }
        #endregion

        #region Fields
        private IUnityInstanceIdentifier unityInstanceIdentifier;
        private string rootPersistentPath;
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
            
            #region Internal Managers Setup
            #region Pre-Initialization
            Debug.Log($"[MainManager] Pre-Initializing internal manager instances.");
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.PreInitializeInstance();
            }
            Debug.Log($"[MainManager] Pre-Initialized internal manager instances.");
            #endregion

            #region Initialization
            Debug.Log($"[MainManager] Initializing internal manager instances.");
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.InitializeInstance();
            }
            Debug.Log($"[MainManager] Initialized internal manager instances.");
            #endregion

            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            CSharpInstanceManager csharpInstanceManager = CSharpInstanceManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;
            
            #region Loo Cast Registration
            Namespace looCastNamespace = new Namespace("LooCast");
            namespaceManager.RegisterNamespace(looCastNamespace);
            
            UnityInstanceType mainManagerType = new UnityInstanceType(typeof(MainManager), looCastNamespace);
            typeManager.RegisterType(mainManagerType);
            
            UnityInstance mainManagerInstance = new UnityInstance(this, mainManagerType);
            unityInstanceManager.RegisterUnityInstance(mainManagerInstance);
            
            #region System
            Namespace looCastSystemNamespace = new Namespace("System", looCastNamespace);
            namespaceManager.RegisterNamespace(looCastSystemNamespace);

            CSharpInstanceType iPersistableType = new CSharpInstanceType(typeof(IPersistable), looCastSystemNamespace);
            CSharpInstanceType iNamespaceType = new CSharpInstanceType(typeof(INamespace), looCastSystemNamespace);
            CSharpInstanceType iInstanceType = new CSharpInstanceType(typeof(IInstance), looCastSystemNamespace);
            CSharpInstanceType iCSharpInstanceType = new CSharpInstanceType(typeof(ICSharpInstance), looCastSystemNamespace);
            CSharpInstanceType iUnityInstanceType = new CSharpInstanceType(typeof(IUnityInstance), looCastSystemNamespace);
            CSharpInstanceType iObjectType = new CSharpInstanceType(typeof(IObject), looCastSystemNamespace);
            CSharpInstanceType iGameObjectType = new CSharpInstanceType(typeof(IGameObject), looCastSystemNamespace);
            CSharpInstanceType iComponentType = new CSharpInstanceType(typeof(IComponent), looCastSystemNamespace);
            CSharpInstanceType namespaceType = new CSharpInstanceType(typeof(Namespace), looCastSystemNamespace);
            CSharpInstanceType csharpInstanceType = new CSharpInstanceType(typeof(CSharpInstance), looCastSystemNamespace);
            CSharpInstanceType unityInstanceType = new CSharpInstanceType(typeof(UnityInstance), looCastSystemNamespace);
            CSharpInstanceType extendedMonoBehaviourType = new CSharpInstanceType(typeof(ExtendedMonoBehaviour), looCastSystemNamespace);
            CSharpInstanceType bigFloatType = new CSharpInstanceType(typeof(BigFloat), looCastSystemNamespace);
            CSharpInstanceType bigVector3Type = new CSharpInstanceType(typeof(BigVector3), looCastSystemNamespace);
            typeManager.RegisterType(iPersistableType);
            typeManager.RegisterType(iNamespaceType);
            typeManager.RegisterType(iInstanceType);
            typeManager.RegisterType(iCSharpInstanceType);
            typeManager.RegisterType(iUnityInstanceType);
            typeManager.RegisterType(iObjectType);
            typeManager.RegisterType(iGameObjectType);
            typeManager.RegisterType(iComponentType);
            typeManager.RegisterType(namespaceType);
            typeManager.RegisterType(csharpInstanceType);
            typeManager.RegisterType(unityInstanceType);
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

            CSharpInstanceType iIdentifierType = new CSharpInstanceType(typeof(IIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iIdentifiableType = new CSharpInstanceType(typeof(IIdentifiable), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iIdentifierType);
            typeManager.RegisterType(iIdentifiableType);
            
            CSharpInstanceType iRegistryIdentifierType = new CSharpInstanceType(typeof(IRegistryIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iRegistryIdentifiableType = new CSharpInstanceType(typeof(IRegistryIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType registryIdentifierType = new CSharpInstanceType(typeof(RegistryIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iRegistryIdentifierType);
            typeManager.RegisterType(iRegistryIdentifiableType);
            typeManager.RegisterType(registryIdentifierType);

            CSharpInstanceType iNamespaceIdentifierType = new CSharpInstanceType(typeof(INamespaceIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iNamespaceIdentifiableType = new CSharpInstanceType(typeof(INamespaceIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType namespaceIdentifierType = new CSharpInstanceType(typeof(NamespaceIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iNamespaceIdentifierType);
            typeManager.RegisterType(iNamespaceIdentifiableType);
            typeManager.RegisterType(namespaceIdentifierType);

            CSharpInstanceType iTypeIdentifierType = new CSharpInstanceType(typeof(ITypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iTypeIdentifiableType = new CSharpInstanceType(typeof(ITypeIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType typeIdentifierType = new CSharpInstanceType(typeof(TypeIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iTypeIdentifierType);
            typeManager.RegisterType(iTypeIdentifiableType);
            typeManager.RegisterType(typeIdentifierType);

            CSharpInstanceType iMetaDataIdentifier = new CSharpInstanceType(typeof(IMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iMetaDataIdentifiable = new CSharpInstanceType(typeof(IMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType metaDataIdentifier = new CSharpInstanceType(typeof(MetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iMetaDataIdentifier);
            typeManager.RegisterType(iMetaDataIdentifiable);
            typeManager.RegisterType(iMetaDataTypeIdentifier);
            typeManager.RegisterType(metaDataIdentifier);

            CSharpInstanceType iDataIdentifier = new CSharpInstanceType(typeof(IDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataIdentifiable = new CSharpInstanceType(typeof(IDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataTypeIdentifier = new CSharpInstanceType(typeof(IDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType dataIdentifier = new CSharpInstanceType(typeof(DataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iDataIdentifier);
            typeManager.RegisterType(iDataIdentifiable);
            typeManager.RegisterType(iDataTypeIdentifier);
            typeManager.RegisterType(dataIdentifier);

            CSharpInstanceType iDataObjectIdentifier = new CSharpInstanceType(typeof(IDataObjectIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataObjectIdentifiable = new CSharpInstanceType(typeof(IDataObjectIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataObjectTypeIdentifier = new CSharpInstanceType(typeof(IDataObjectTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataObjectMetaDataIdentifier = new CSharpInstanceType(typeof(IDataObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataObjectMetaDataIdentifiable = new CSharpInstanceType(typeof(IDataObjectMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataObjectMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IDataObjectMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType dataObjectIdentifier = new CSharpInstanceType(typeof(DataObjectIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType dataObjectMetaDataIdentifier = new CSharpInstanceType(typeof(DataObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iDataObjectIdentifier);
            typeManager.RegisterType(iDataObjectIdentifiable);
            typeManager.RegisterType(iDataObjectTypeIdentifier);
            typeManager.RegisterType(iDataObjectMetaDataIdentifier);
            typeManager.RegisterType(iDataObjectMetaDataIdentifiable);
            typeManager.RegisterType(iDataObjectMetaDataTypeIdentifier);
            typeManager.RegisterType(dataObjectIdentifier);
            typeManager.RegisterType(dataObjectMetaDataIdentifier);

            CSharpInstanceType iDataFileIdentifier = new CSharpInstanceType(typeof(IDataFileIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataFileIdentifiable = new CSharpInstanceType(typeof(IDataFileIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataFileTypeIdentifier = new CSharpInstanceType(typeof(IDataFileTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataFileMetaDataIdentifier = new CSharpInstanceType(typeof(IDataFileMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataFileMetaDataIdentifiable = new CSharpInstanceType(typeof(IDataFileMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataFileMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IDataFileMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType dataFileIdentifier = new CSharpInstanceType(typeof(DataFileIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType dataFileMetaDataIdentifier = new CSharpInstanceType(typeof(DataFileMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iDataFileIdentifier);
            typeManager.RegisterType(iDataFileIdentifiable);
            typeManager.RegisterType(iDataFileTypeIdentifier);
            typeManager.RegisterType(iDataFileMetaDataIdentifier);
            typeManager.RegisterType(iDataFileMetaDataIdentifiable);
            typeManager.RegisterType(iDataFileMetaDataTypeIdentifier);
            typeManager.RegisterType(dataFileIdentifier);
            typeManager.RegisterType(dataFileMetaDataIdentifier);

            CSharpInstanceType iDataFolderIdentifier = new CSharpInstanceType(typeof(IDataFolderIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataFolderIdentifiable = new CSharpInstanceType(typeof(IDataFolderIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataFolderTypeIdentifier = new CSharpInstanceType(typeof(IDataFolderTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataFolderMetaDataIdentifier = new CSharpInstanceType(typeof(IDataFolderMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataFolderMetaDataIdentifiable = new CSharpInstanceType(typeof(IDataFolderMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iDataFolderMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IDataFolderMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType dataFolderIdentifier = new CSharpInstanceType(typeof(DataFolderIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType dataFolderMetaDataIdentifier = new CSharpInstanceType(typeof(DataFolderMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iDataFolderIdentifier);
            typeManager.RegisterType(iDataFolderIdentifiable);
            typeManager.RegisterType(iDataFolderTypeIdentifier);
            typeManager.RegisterType(iDataFolderMetaDataIdentifier);
            typeManager.RegisterType(iDataFolderMetaDataIdentifiable);
            typeManager.RegisterType(iDataFolderMetaDataTypeIdentifier);
            typeManager.RegisterType(dataFolderIdentifier);
            typeManager.RegisterType(dataFolderMetaDataIdentifier);

            CSharpInstanceType iInstanceIdentifier = new CSharpInstanceType(typeof(IInstanceIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iInstanceIdentifiable = new CSharpInstanceType(typeof(IInstanceIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iInstanceTypeIdentifier = new CSharpInstanceType(typeof(IInstanceTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iInstanceDataIdentifier = new CSharpInstanceType(typeof(IInstanceDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iInstanceDataIdentifiable = new CSharpInstanceType(typeof(IInstanceDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iInstanceDataTypeIdentifier = new CSharpInstanceType(typeof(IInstanceDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iInstanceMetaDataIdentifier = new CSharpInstanceType(typeof(IInstanceMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iInstanceMetaDataIdentifiable = new CSharpInstanceType(typeof(IInstanceMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iInstanceMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IInstanceMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType instanceIdentifier = new CSharpInstanceType(typeof(InstanceIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType instanceDataIdentifier = new CSharpInstanceType(typeof(InstanceDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType instanceMetaDataIdentifier = new CSharpInstanceType(typeof(InstanceMetaDataIdentifier), looCastSystemIdentificationNamespace);
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

            CSharpInstanceType iCSharpInstanceIdentifier = new CSharpInstanceType(typeof(ICSharpInstanceIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iCSharpInstanceIdentifiable = new CSharpInstanceType(typeof(ICSharpInstanceIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iCSharpInstanceTypeIdentifier = new CSharpInstanceType(typeof(ICSharpInstanceTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iCSharpInstanceDataIdentifier = new CSharpInstanceType(typeof(ICSharpInstanceDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iCSharpInstanceDataIdentifiable = new CSharpInstanceType(typeof(ICSharpInstanceDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iCSharpInstanceDataTypeIdentifier = new CSharpInstanceType(typeof(ICSharpInstanceDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iCSharpInstanceMetaDataIdentifier = new CSharpInstanceType(typeof(ICSharpInstanceMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iCSharpInstanceMetaDataIdentifiable = new CSharpInstanceType(typeof(ICSharpInstanceMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iCSharpInstanceMetaDataTypeIdentifier = new CSharpInstanceType(typeof(ICSharpInstanceMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType csharpInstanceIdentifier = new CSharpInstanceType(typeof(SystemObjectIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType csharpInstanceDataIdentifier = new CSharpInstanceType(typeof(CSharpInstanceDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType csharpInstanceMetaDataIdentifier = new CSharpInstanceType(typeof(CSharpInstanceMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iCSharpInstanceIdentifier);
            typeManager.RegisterType(iCSharpInstanceIdentifiable);
            typeManager.RegisterType(iCSharpInstanceTypeIdentifier);
            typeManager.RegisterType(iCSharpInstanceDataIdentifier);
            typeManager.RegisterType(iCSharpInstanceDataIdentifiable);
            typeManager.RegisterType(iCSharpInstanceDataTypeIdentifier);
            typeManager.RegisterType(iCSharpInstanceMetaDataIdentifier);
            typeManager.RegisterType(iCSharpInstanceMetaDataIdentifiable);
            typeManager.RegisterType(iCSharpInstanceMetaDataTypeIdentifier);
            typeManager.RegisterType(csharpInstanceIdentifier);
            typeManager.RegisterType(csharpInstanceDataIdentifier);
            typeManager.RegisterType(csharpInstanceMetaDataIdentifier);

            CSharpInstanceType iUnityInstanceIdentifier = new CSharpInstanceType(typeof(IUnityInstanceIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iUnityInstanceIdentifiable = new CSharpInstanceType(typeof(IUnityInstanceIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iUnityInstanceTypeIdentifier = new CSharpInstanceType(typeof(IUnityInstanceTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iUnityInstanceDataIdentifier = new CSharpInstanceType(typeof(IUnityInstanceDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iUnityInstanceDataIdentifiable = new CSharpInstanceType(typeof(IUnityInstanceDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iUnityInstanceDataTypeIdentifier = new CSharpInstanceType(typeof(IUnityInstanceDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iUnityInstanceMetaDataIdentifier = new CSharpInstanceType(typeof(IUnityInstanceMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iUnityInstanceMetaDataIdentifiable = new CSharpInstanceType(typeof(IUnityInstanceMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iUnityInstanceMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IUnityInstanceMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType unityInstanceIdentifier = new CSharpInstanceType(typeof(GameObjectIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType unityInstanceDataIdentifier = new CSharpInstanceType(typeof(UnityInstanceDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType unityInstanceMetaDataIdentifier = new CSharpInstanceType(typeof(UnityInstanceMetaDataIdentifier), looCastSystemIdentificationNamespace);
            typeManager.RegisterType(iUnityInstanceIdentifier);
            typeManager.RegisterType(iUnityInstanceIdentifiable);
            typeManager.RegisterType(iUnityInstanceTypeIdentifier);
            typeManager.RegisterType(iUnityInstanceDataIdentifier);
            typeManager.RegisterType(iUnityInstanceDataIdentifiable);
            typeManager.RegisterType(iUnityInstanceDataTypeIdentifier);
            typeManager.RegisterType(iUnityInstanceMetaDataIdentifier);
            typeManager.RegisterType(iUnityInstanceMetaDataIdentifiable);
            typeManager.RegisterType(iUnityInstanceMetaDataTypeIdentifier);
            typeManager.RegisterType(unityInstanceIdentifier);
            typeManager.RegisterType(unityInstanceDataIdentifier);
            typeManager.RegisterType(unityInstanceMetaDataIdentifier);

            CSharpInstanceType iObjectIdentifier = new CSharpInstanceType(typeof(IObjectIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iObjectIdentifiable = new CSharpInstanceType(typeof(IObjectIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iObjectTypeIdentifier = new CSharpInstanceType(typeof(IObjectTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iObjectDataIdentifier = new CSharpInstanceType(typeof(IObjectDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iObjectDataIdentifiable = new CSharpInstanceType(typeof(IObjectDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iObjectDataTypeIdentifier = new CSharpInstanceType(typeof(IObjectDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iObjectMetaDataIdentifier = new CSharpInstanceType(typeof(IObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iObjectMetaDataIdentifiable = new CSharpInstanceType(typeof(IObjectMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iObjectMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IObjectMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType objectIdentifier = new CSharpInstanceType(typeof(ObjectIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType objectDataIdentifier = new CSharpInstanceType(typeof(ObjectDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType objectMetaDataIdentifier = new CSharpInstanceType(typeof(ObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
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

            CSharpInstanceType iGameObjectIdentifier = new CSharpInstanceType(typeof(IGameObjectIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iGameObjectIdentifiable = new CSharpInstanceType(typeof(IGameObjectIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iGameObjectTypeIdentifier = new CSharpInstanceType(typeof(IGameObjectTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iGameObjectDataIdentifier = new CSharpInstanceType(typeof(IGameObjectDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iGameObjectDataIdentifiable = new CSharpInstanceType(typeof(IGameObjectDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iGameObjectDataTypeIdentifier = new CSharpInstanceType(typeof(IGameObjectDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iGameObjectMetaDataIdentifier = new CSharpInstanceType(typeof(IGameObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iGameObjectMetaDataIdentifiable = new CSharpInstanceType(typeof(IGameObjectMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iGameObjectMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IGameObjectMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType gameObjectIdentifier = new CSharpInstanceType(typeof(GameObjectIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType gameObjectDataIdentifier = new CSharpInstanceType(typeof(GameObjectDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType gameObjectMetaDataIdentifier = new CSharpInstanceType(typeof(GameObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
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

            CSharpInstanceType iComponentIdentifier = new CSharpInstanceType(typeof(IComponentIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iComponentIdentifiable = new CSharpInstanceType(typeof(IComponentIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iComponentTypeIdentifier = new CSharpInstanceType(typeof(IComponentTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iComponentDataIdentifier = new CSharpInstanceType(typeof(IComponentDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iComponentDataIdentifiable = new CSharpInstanceType(typeof(IComponentDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iComponentDataTypeIdentifier = new CSharpInstanceType(typeof(IComponentDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iComponentMetaDataIdentifier = new CSharpInstanceType(typeof(IComponentMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iComponentMetaDataIdentifiable = new CSharpInstanceType(typeof(IComponentMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iComponentMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IComponentMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType componentIdentifier = new CSharpInstanceType(typeof(ComponentIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType componentDataIdentifier = new CSharpInstanceType(typeof(ComponentDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType componentMetaDataIdentifier = new CSharpInstanceType(typeof(ComponentMetaDataIdentifier), looCastSystemIdentificationNamespace);
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

            CSharpInstanceType iResourceIdentifier = new CSharpInstanceType(typeof(IResourceIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceIdentifiable = new CSharpInstanceType(typeof(IResourceIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceTypeIdentifier = new CSharpInstanceType(typeof(IResourceTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceDataIdentifier = new CSharpInstanceType(typeof(IResourceDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceDataIdentifiable = new CSharpInstanceType(typeof(IResourceDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceDataTypeIdentifier = new CSharpInstanceType(typeof(IResourceDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceMetaDataIdentifier = new CSharpInstanceType(typeof(IResourceMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceMetaDataIdentifiable = new CSharpInstanceType(typeof(IResourceMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IResourceMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceIdentifier = new CSharpInstanceType(typeof(ResourceIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceDataIdentifier = new CSharpInstanceType(typeof(ResourceDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceMetaDataIdentifier = new CSharpInstanceType(typeof(ResourceMetaDataIdentifier), looCastSystemIdentificationNamespace);
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

            CSharpInstanceType iResourceObjectIdentifier = new CSharpInstanceType(typeof(IResourceObjectIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceObjectIdentifiable = new CSharpInstanceType(typeof(IResourceObjectIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceObjectTypeIdentifier = new CSharpInstanceType(typeof(IResourceObjectTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceObjectDataIdentifier = new CSharpInstanceType(typeof(IResourceObjectDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceObjectDataIdentifiable = new CSharpInstanceType(typeof(IResourceObjectDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceObjectDataTypeIdentifier = new CSharpInstanceType(typeof(IResourceObjectDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceObjectMetaDataIdentifier = new CSharpInstanceType(typeof(IResourceObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceObjectMetaDataIdentifiable = new CSharpInstanceType(typeof(IResourceObjectMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceObjectMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IResourceObjectMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceObjectIdentifier = new CSharpInstanceType(typeof(ResourceObjectIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceObjectDataIdentifier = new CSharpInstanceType(typeof(ResourceObjectDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceObjectMetaDataIdentifier = new CSharpInstanceType(typeof(ResourceObjectMetaDataIdentifier), looCastSystemIdentificationNamespace);
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

            CSharpInstanceType iResourceFileIdentifier = new CSharpInstanceType(typeof(IResourceFileIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFileIdentifiable = new CSharpInstanceType(typeof(IResourceFileIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFileTypeIdentifier = new CSharpInstanceType(typeof(IResourceFileTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFileDataIdentifier = new CSharpInstanceType(typeof(IResourceFileDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFileDataIdentifiable = new CSharpInstanceType(typeof(IResourceFileDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFileDataTypeIdentifier = new CSharpInstanceType(typeof(IResourceFileDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFileMetaDataIdentifier = new CSharpInstanceType(typeof(IResourceFileMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFileMetaDataIdentifiable = new CSharpInstanceType(typeof(IResourceFileMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFileMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IResourceFileMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceFileIdentifier = new CSharpInstanceType(typeof(ResourceFileIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceFileDataIdentifier = new CSharpInstanceType(typeof(ResourceFileDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceFileMetaDataIdentifier = new CSharpInstanceType(typeof(ResourceFileMetaDataIdentifier), looCastSystemIdentificationNamespace);
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

            CSharpInstanceType iResourceFolderIdentifier = new CSharpInstanceType(typeof(IResourceFolderIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFolderIdentifiable = new CSharpInstanceType(typeof(IResourceFolderIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFolderTypeIdentifier = new CSharpInstanceType(typeof(IResourceFolderTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFolderDataIdentifier = new CSharpInstanceType(typeof(IResourceFolderDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFolderDataIdentifiable = new CSharpInstanceType(typeof(IResourceFolderDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFolderDataTypeIdentifier = new CSharpInstanceType(typeof(IResourceFolderDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFolderMetaDataIdentifier = new CSharpInstanceType(typeof(IResourceFolderMetaDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFolderMetaDataIdentifiable = new CSharpInstanceType(typeof(IResourceFolderMetaDataIdentifiable), looCastSystemIdentificationNamespace);
            CSharpInstanceType iResourceFolderMetaDataTypeIdentifier = new CSharpInstanceType(typeof(IResourceFolderMetaDataTypeIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceFolderIdentifier = new CSharpInstanceType(typeof(ResourceFolderIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceFolderDataIdentifier = new CSharpInstanceType(typeof(ResourceFolderDataIdentifier), looCastSystemIdentificationNamespace);
            CSharpInstanceType resourceFolderMetaDataIdentifier = new CSharpInstanceType(typeof(ResourceFolderMetaDataIdentifier), looCastSystemIdentificationNamespace);
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

            #region Post-Initialization
            Debug.Log($"[MainManager] Post-Initializing internal manager instances.");
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.PostInitializeInstance();
            }
            Debug.Log($"[MainManager] Post-Initialized internal manager instances.");
            #endregion

            #endregion

            #region Core Module Managers Setup
            
            #region Pre-Initialization
            Debug.Log($"[MainManager] Pre-Initializing core module manager instances.");
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PreInitializeInstance();
            }
            Debug.Log($"[MainManager] Pre-Initialized core module manager instances.");
            #endregion

            #region Initialization
            Debug.Log($"[MainManager] Initializing core module manager instances.");
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.InitializeInstance();
            }
            Debug.Log($"[MainManager] Initialized core module manager instances.");
            #endregion

            #region Post-Initialization
            Debug.Log($"[MainManager] Post-Initializing core module manager instances.");
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PostInitializeInstance();
            }
            Debug.Log($"[MainManager] Post-Initialized core module manager instances.");
            #endregion

            #endregion

            IsEarlyPreInitializing = true;
            Debug.Log($"[MainManager] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            #region Early Pre-Initialization

            #region Main Manager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPreInitialize();
            }
            #endregion

            #endregion

            IsEarlyPreInitializing = false;
            IsEarlyPreInitialized = true;
            Debug.Log($"[MainManager] Finished Early Pre-Initialization in Scene '{activeSceneName}'.");

            PreInitialize();
        }

        private void PreInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPreInitializing = true;
            Debug.Log($"[MainManager] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            #region Pre-Initialization

            #region Main Manager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PreInitialize();
            }
            #endregion

            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;
            Debug.Log($"[MainManager] Finished Pre-Initialization in Scene '{activeSceneName}'.");

            LatePreInitialize();
        }

        private void LatePreInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePreInitializing = true;
            Debug.Log($"[MainManager] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            #region Late Pre-Initialization

            #region MainManager
            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePreInitialize();
            }
            #endregion

            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;
            Debug.Log($"[MainManager] Finished Pre-Initialization in Scene '{activeSceneName}'.");

            _ = Instance;
        }

        private void EarlyInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyInitializing = true;
            Debug.Log($"[MainManager] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            #region Early Initialization

            #region Main Manager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyInitialize();
            }
            #endregion

            #endregion

            IsEarlyInitializing = false;
            IsEarlyInitialized = true;
            Debug.Log($"[MainManager] Finished Early Pre-Initialization in Scene '{activeSceneName}'.");

            Initialize();
        }

        private void Initialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsInitializing = true;
            Debug.Log($"[MainManager] Starting Initialization in Scene '{activeSceneName}'.");

            #region Initialization

            #region MainManager
            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.Initialize();
            }
            #endregion

            #region DEPRECATED! [TO BE MOVED!]

            #region SteamManager
            _ = SteamworksManager.Initialized;
            #endregion

            #region Data.Path
            _ = Data.Data.Path;
            #endregion

            #region TimerUtil
            TimerUtil.InitializeInstance();
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

            #endregion

            IsInitializing = false;
            IsInitialized = true;
            Debug.Log($"[MainManager] Finished Initialization in Scene '{activeSceneName}'.");

            LateInitialize();
        }

        private void LateInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLateInitializing = true;
            Debug.Log($"[MainManager] Starting Late Pre-Initialization in Scene '{activeSceneName}'.");

            #region Late Initialization

            #region Main Manager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LateInitialize();
            }
            #endregion

            #endregion

            IsLateInitializing = false;
            IsLateInitialized = true;
            Debug.Log($"[MainManager] Finished Late Pre-Initialization in Scene '{activeSceneName}'.");
        }

        private void EarlyPostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPostInitializing = true;
            Debug.Log($"[MainManager] Starting Early Post-Initialization in Scene '{activeSceneName}'.");

            #region Early Post-Initialization

            #region MainManager

            #endregion

            #region Core Module Managers
            LooCast.Core.CoreManager looCastCoreManager = LooCast.Core.CoreManager.Instance;
            looCastCoreManager.PostInitialize();
            #endregion

            #endregion

            IsEarlyPostInitializing = false;
            IsEarlyPostInitialized = true;
            Debug.Log($"[MainManager] Finished Early Post-Initialization in Scene '{activeSceneName}'.");

            PostInitialize();
        }

        private void PostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPostInitializing = true;
            Debug.Log($"[MainManager] Starting Post-Initialization in Scene '{activeSceneName}'.");

            #region Post-Initialization

            #region MainManager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PostInitialize();
            }
            #endregion

            #endregion

            IsPostInitializing = false;
            IsPostInitialized = true;
            Debug.Log($"[MainManager] Finished Post-Initialization in Scene '{activeSceneName}'.");

            LatePostInitialize();
        }

        private void LatePostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePostInitializing = true;
            Debug.Log($"[MainManager] Starting Late Post-Initialization in Scene '{activeSceneName}'.");

            #region Late Post-Initialization

            #region MainManager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePostInitialize();
            }
            #endregion

            #endregion

            IsLatePostInitializing = false;
            IsLatePostInitialized = true;
            Debug.Log($"[MainManager] Finished Late Post-Initialization in Scene '{activeSceneName}'.");
        }
        #endregion

        #region Termination Phases
        private void EarlyPreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPreTerminating = true;
            Debug.Log($"[MainManager] Starting Early Pre-Termination in Scene '{activeSceneName}'.");

            #region Early Pre-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.EarlyPreTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsEarlyPreTerminating = false;
            IsEarlyPreTerminated = true;
            Debug.Log($"[MainManager] Finished Early Pre-Termination in Scene '{activeSceneName}'.");

            PreTerminate();
        }

        private void PreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPreTerminating = true;
            Debug.Log($"[MainManager] Starting Pre-Termination in Scene '{activeSceneName}'.");

            #region Pre-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.PreTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsPreTerminating = false;
            IsPreTerminated = true;
            Debug.Log($"[MainManager] Finished Pre-Termination in Scene '{activeSceneName}'.");

            LatePreTerminate();
        }

        private void LatePreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePreTerminating = true;
            Debug.Log($"[MainManager] Starting Late Pre-Termination in Scene '{activeSceneName}'.");

            #region Late Pre-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.LatePreTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsLatePreTerminating = false;
            IsLatePreTerminated = true;
            Debug.Log($"[MainManager] Finished Late Pre-Termination in Scene '{activeSceneName}'.");

            EarlyTerminate();
        }

        private void EarlyTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyTerminating = true;
            Debug.Log($"[MainManager] Starting Early Termination in Scene '{activeSceneName}'.");

            #region Early Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.EarlyTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsEarlyTerminating = false;
            IsEarlyTerminated = true;
            Debug.Log($"[MainManager] Finished Early Termination in Scene '{activeSceneName}'.");

            Terminate();
        }

        private void Terminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsTerminating = true;
            Debug.Log($"[MainManager] Starting Termination in Scene '{activeSceneName}'.");

            #region Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.Terminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsTerminating = false;
            IsTerminated = true;
            Debug.Log($"[MainManager] Finished Termination in Scene '{activeSceneName}'.");

            LateTerminate();
        }

        private void LateTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLateTerminating = true;
            Debug.Log($"[MainManager] Starting Late Termination in Scene '{activeSceneName}'.");

            #region Late Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.LateTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsLateTerminating = false;
            IsLateTerminated = true;
            Debug.Log($"[MainManager] Finished Late Termination in Scene '{activeSceneName}'.");

            EarlyPostTerminate();
        }

        private void EarlyPostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPostTerminating = true;
            Debug.Log($"[MainManager] Starting Early Post-Termination in Scene '{activeSceneName}'.");

            #region Early Post-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.EarlyPostTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsEarlyPostTerminating = false;
            IsEarlyPostTerminated = true;
            Debug.Log($"[MainManager] Finished Early Post-Termination in Scene '{activeSceneName}'.");

            PostTerminate();
        }

        private void PostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPostTerminating = true;
            Debug.Log($"[MainManager] Starting Post-Termination in Scene '{activeSceneName}'.");

            #region Post-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.PostTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsPostTerminating = false;
            IsPostTerminated = true;
            Debug.Log($"[MainManager] Finished Post-Termination in Scene '{activeSceneName}'.");

            LatePostTerminate();
        }

        private void LatePostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePostTerminating = true;
            Debug.Log($"[MainManager] Starting Late Post-Termination in Scene '{activeSceneName}'.");

            #region Late Post-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.LatePostTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsLatePostTerminating = false;
            IsLatePostTerminated = true;
            Debug.Log($"[MainManager] Finished Late Post-Termination in Scene '{activeSceneName}'.");
        }
        #endregion

        #endregion
    }
}
