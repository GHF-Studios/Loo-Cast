using System;
using System.Collections.Generic;

namespace LooCast.Universe
{
    using LooCast.System;
    using LooCast.System.ECS;
    using LooCast.System.Lua;
    using LooCast.Core;

    [LuaNamespace("Universe")]
    public sealed class UniverseManager : ModuleManager
    {
        #region Static Properties
        public static UniverseManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<UniverseManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static UniverseManager instance;
        #endregion

        #region Fields
        private Universe universe;
        private UniverseObserver universeObserver;
        #endregion

        #region Constructors
        public UniverseManager() : base()
        {
            // Add pre-included components here
            
            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedUniverseManagerEntityTypeName = typeof(UniverseManager).AssemblyQualifiedName;
                string assemblyQualifiedUniverseManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedUniverseManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData instanceMetaData = new Entity.MetaData();
                instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedUniverseManagerEntityTypeName;
                instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedUniverseManagerEntityMetaDataTypeName;
                instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedUniverseManagerEntityDataTypeName;
                instanceMetaData.EntityID = new Guid();

                Manager.Data instanceData = new Manager.Data();
                instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedUniverseManagerEntityTypeName;
                instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedUniverseManagerEntityMetaDataTypeName;
                instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedUniverseManagerEntityDataTypeName;
                instanceData.ManagerName = "UniverseManager";
                instanceData.ManagerParent = LooCastCoreManager.Instance;

                SetEntityMetaData(instanceMetaData);
                SetEntityData(instanceData);

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                // Set pre-included components' metaData here

                // Set pre-included component's data here

                // Register pre-included components here

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPostSetup();
                }
            });

            RegisterPreInitializationAction(() =>
            {
                // Pre-Initialize pre-included components here
            });

            RegisterInitializationAction(() =>
            {
                // Initialize pre-included components here
            });

            RegisterPostInitializationAction(() =>
            {
                // Post-Initialize pre-included components here
            });
        }
        #endregion

        #region Static Methods
        [LuaMethod("CreateUniverse")]
        public static Universe CreateUniverse(int chunkSize)
        {
            Universe universe = new Universe(chunkSize);
            UnityEngine.Debug.Log("[Universe] Created universe!");
            return universe;
        }
        
        [LuaMethod("CreateUniverseObserver")]
        public static UniverseObserver CreateUniverseObserver(int observingDistance)
        {
            UniverseObserver universeObserver = new UniverseObserver(observingDistance);
            UnityEngine.Debug.Log("[UniverseObserver] Created universe observer!");
            return universeObserver;
        }

        [LuaMethod("GetUniverse")]
        public static Universe GetUniverse()
        {
            return Instance.universe;
        }

        [LuaMethod("GetUniverseObserver")]
        public static UniverseObserver GetUniverseObserver()
        {
            return Instance.universeObserver;
        }

        [LuaMethod("SetUniverse")]
        public static void SetUniverse(Universe universe)
        {
            if (Instance.universe != null)
            {
                throw new InvalidOperationException($"Universe has already been set!");
            }

            Instance.universe = universe;
            UnityEngine.Debug.Log("[LooCastCoreManager] Set universe!");
        }

        [LuaMethod("SetUniverseObserver")]
        public static void SetUniverseObserver(UniverseObserver universeObserver)
        {
            if (Instance.universeObserver != null)
            {
                throw new InvalidOperationException($"UniverseObserver has already been set!");
            }

            Instance.universeObserver = universeObserver;
            UnityEngine.Debug.Log("[LooCastCoreManager] Set universe observer!");
        }
        #endregion
    }
}
