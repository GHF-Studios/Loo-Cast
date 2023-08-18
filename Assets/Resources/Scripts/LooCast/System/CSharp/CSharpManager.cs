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
        public static event Action<IEnumerable<TypeInfo>> OnTypesRegistered;
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
        private Dictionary<Type, TypeInfo> registeredTypes;
        #endregion

        #region Constructors
        public CSharpManager() : base()
        {
            registeredTypes = new Dictionary<Type, TypeInfo>();
            
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
                RegisterTypes(AppDomain.CurrentDomain.GetAssemblies().SelectMany(assembly => assembly.GetTypes()));
                
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
        public static void RegisterTypes(IEnumerable<Type> types)
        {
            Dictionary<Type, TypeInfo> newlyRegisteredTypes = new Dictionary<Type, TypeInfo>();
            foreach (Type type in types)
            {
                newlyRegisteredTypes.Add(type, new TypeInfo(type));
            }

            Instance.registeredTypes = Instance.registeredTypes.Concat(newlyRegisteredTypes).ToDictionary(pair => pair.Key, pair => pair.Value);

            OnTypesRegistered?.Invoke(newlyRegisteredTypes.Values);
        }

        public static TypeInfo GetTypeInfo(Type type)
        {
            if (!Instance.registeredTypes.TryGetValue(type, out TypeInfo typeInfo))
            {
                typeInfo = new TypeInfo(type);
                Instance.registeredTypes.Add(type, typeInfo);
            }
            
            return typeInfo;
        }
        #endregion
    }
}
