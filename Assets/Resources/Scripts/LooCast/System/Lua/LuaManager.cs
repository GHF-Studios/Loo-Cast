using System;
using System.Linq;
using System.Diagnostics;
using System.Collections.Generic;
using NLua;
using NLua.Exceptions;

namespace LooCast.System.Lua
{
    using LooCast.System;
    using LooCast.System.CSharp;
    using LooCast.System.ECS;
    using LooCast.Core;

    [LuaNamespace("Lua")]
    public sealed class LuaManager : ModuleManager
    {
        #region Static Properties
        public static LuaManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<LuaManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static LuaManager instance;
        #endregion

        #region Fields
        private NLua.Lua lua;
        #endregion

        #region Constructors
        public LuaManager() : base()
        {
            lua = new NLua.Lua();
            lua.LoadCLRPackage();

            lua.DoString(@"
            LooCast = {}
            LooCast.System = {}
            LooCast.Universe = {}
            ");

            Type looCastApplicationType = typeof(LooCastApplication);
            lua.RegisterFunction("LooCast.Log", looCastApplicationType.GetMethod("Log"));

            Type universeManagerType = typeof(Universe.UniverseManager);
            lua.RegisterFunction("LooCast.Universe.CreateUniverse", universeManagerType.GetMethod("CreateUniverse"));
            lua.RegisterFunction("LooCast.Universe.CreateUniverseObserver", universeManagerType.GetMethod("CreateUniverseObserver"));
            lua.RegisterFunction("LooCast.Universe.SetUniverse", universeManagerType.GetMethod("SetUniverse"));
            lua.RegisterFunction("LooCast.Universe.SetUniverseObserver", universeManagerType.GetMethod("SetUniverseObserver"));
            lua.RegisterFunction("LooCast.Universe.GetUniverse", universeManagerType.GetMethod("GetUniverse"));
            lua.RegisterFunction("LooCast.Universe.GetUniverseObserver", universeManagerType.GetMethod("GetUniverseObserver"));

            Type universeType = typeof(Universe.Universe);
            lua.RegisterFunction("LooCast.Universe.GetChunkSize", universeType.GetMethod("GetChunkSize"));

            Type universeObserverType = typeof(Universe.UniverseObserver);
            lua.RegisterFunction("LooCast.Universe.GetObservingDistance", universeObserverType.GetMethod("GetObservingDistance"));
            
            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedLuaManagerEntityTypeName = typeof(LuaManager).AssemblyQualifiedName;
                string assemblyQualifiedLuaManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedLuaManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData instanceMetaData = new Entity.MetaData();
                instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedLuaManagerEntityTypeName;
                instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedLuaManagerEntityMetaDataTypeName;
                instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedLuaManagerEntityDataTypeName;
                instanceMetaData.EntityID = new Guid();

                Manager.Data instanceData = new Manager.Data();
                instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedLuaManagerEntityTypeName;
                instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedLuaManagerEntityMetaDataTypeName;
                instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedLuaManagerEntityDataTypeName;
                instanceData.ManagerName = "LuaManager";
                instanceData.ManagerParent = SystemManager.Instance;

                SetEntityMetaData(instanceMetaData);
                SetEntityData(instanceData);

                CSharpManager.OnTypesRegistered += (types) =>
                {
                    RegisterTypes(types);
                };

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
        public static void ExecuteLuaString(string luaString)
        {
            try
            {
                Instance.lua.DoString(luaString);
            }
            catch (LuaException ex)
            {
                LooCastApplication.Log($"[LuaManager] Error executing Lua code: {ex.Message}");
            }
        }
        #endregion

        #region Methods
        /// <summary>
        /// Registers a collection of types to the lua manager.
        /// For performance optimization, it's recommended to register many types simultaneously.
        /// The method analyzes each type and extracts all LuaNamespace and LuaMethod atributes and then proceeds to register those namespaces and methods with the lua api.
        /// </summary>
        /// <param name="types">The types to be registered.</param>
        public void RegisterTypes(IEnumerable<Type> types)
        {
            Stopwatch stopwatch = new Stopwatch();
            stopwatch.Start();

            // TODO: Implement

            stopwatch.Stop();
            UnityEngine.Debug.Log($"[LuaManager] Registering {types.Count()} types took {stopwatch.ElapsedMilliseconds}ms");
        }
        #endregion
    }
}
