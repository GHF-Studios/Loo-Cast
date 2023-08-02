using System;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Serialization;
    
    public sealed class SystemManager : CoreModuleManager
    {
        #region Static Properties
        public static SystemManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<SystemManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static SystemManager instance;
        #endregion

        #region Constructors
        public SystemManager() : base()
        {
            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedSystemManagerEntityTypeName = typeof(SystemManager).AssemblyQualifiedName;
                string assemblyQualifiedSystemManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedSystemManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData systemManagerMetaData = new Entity.MetaData();
                systemManagerMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedSystemManagerEntityTypeName;
                systemManagerMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedSystemManagerEntityMetaDataTypeName;
                systemManagerMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedSystemManagerEntityDataTypeName;
                systemManagerMetaData.EntityID = new Guid();

                Manager.Data systemManagerData = new Manager.Data();
                systemManagerData.AssemblyQualifiedEntityTypeName = assemblyQualifiedSystemManagerEntityTypeName;
                systemManagerData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedSystemManagerEntityMetaDataTypeName;
                systemManagerData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedSystemManagerEntityDataTypeName;
                systemManagerData.ManagerName = "SystemManager";
                systemManagerData.ManagerParent = MainManager.Instance;

                SetEntityMetaData(systemManagerMetaData);
                SetEntityData(systemManagerData);

                moduleManagerChildrenList.Add(SerializationManager.Instance);
                moduleManagerChildrenList.Add(EntityManager.Instance);
                moduleManagerChildrenList.Add(ComponentManager.Instance);

                foreach (IModuleManager moduleManager in moduleManagerChildrenList)
                {
                    moduleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                // Set pre-included components' metaData here

                // Set pre-included component's data here

                // Register pre-included components here

                foreach (IModuleManager moduleManager in moduleManagerChildrenList)
                {
                    moduleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (IModuleManager moduleManager in moduleManagerChildrenList)
                {
                    moduleManager.OnPostSetup();
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
    }
}
