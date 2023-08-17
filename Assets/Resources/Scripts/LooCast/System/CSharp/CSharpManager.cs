using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;

namespace LooCast.System.CSharp
{
    using LooCast.System;
    using LooCast.System.ECS;
    using LooCast.Core;

    public sealed class CSharpManager : ModuleManager
    {
        #region Static Events
        public static event Action<Type[]> OnTypesRegistered;
        #endregion

        #region Static Properties
        public static CSharpManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<CSharpManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static CSharpManager instance;
        #endregion

        #region Fields
        private HashSet<Assembly> assemblies;
        private HashSet<Type> types;
        #endregion

        #region Constructors
        public CSharpManager() : base()
        {
            assemblies = new HashSet<Assembly>();
            types = new HashSet<Type>();
            
            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedCSharpManagerEntityTypeName = typeof(CSharpManager).AssemblyQualifiedName;
                string assemblyQualifiedCSharpManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedCSharpManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData instanceMetaData = new Entity.MetaData();
                instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedCSharpManagerEntityTypeName;
                instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedCSharpManagerEntityMetaDataTypeName;
                instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedCSharpManagerEntityDataTypeName;
                instanceMetaData.EntityID = new Guid();

                Manager.Data instanceData = new Manager.Data();
                instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedCSharpManagerEntityTypeName;
                instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedCSharpManagerEntityMetaDataTypeName;
                instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedCSharpManagerEntityDataTypeName;
                instanceData.ManagerName = "CSharpManager";
                instanceData.ManagerParent = SystemManager.Instance;

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
                RegisterAssemblies(AppDomain.CurrentDomain.GetAssemblies());
                
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
        public static void RegisterAssemblies(params Assembly[] assemblies)
        {
            List<Type> types = new List<Type>();
            
            foreach (Assembly assembly in assemblies)
            {
                Instance.assemblies.Add(assembly);
                types.AddRange(assembly.GetTypes());
            }
            
            RegisterTypes(types.ToArray());
        }

        private static void RegisterTypes(params Type[] types)
        {
            foreach (Type type in types)
            {
                Instance.types.Add(type);
            }
            
            OnTypesRegistered?.Invoke(types);
        }
        #endregion
    }
}
