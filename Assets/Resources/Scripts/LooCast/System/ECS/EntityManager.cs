using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.ECS
{
    public sealed class EntityManager : ModuleManager
    {
        #region Static Properties
        public static EntityManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<EntityManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static EntityManager instance;
        #endregion

        #region Fields
        private Dictionary<Guid, Entity> registeredEntities;
        #endregion

        #region Constructors
        public EntityManager() : base()
        {
            registeredEntities = new Dictionary<Guid, Entity>();

            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedEntityManagerEntityTypeName = typeof(EntityManager).AssemblyQualifiedName;
                string assemblyQualifiedEntityManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedEntityManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData entityManagerMetaData = new Entity.MetaData();
                entityManagerMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedEntityManagerEntityTypeName;
                entityManagerMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedEntityManagerEntityMetaDataTypeName;
                entityManagerMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedEntityManagerEntityDataTypeName;
                entityManagerMetaData.EntityID = new Guid();

                Manager.Data entityManagerData = new Manager.Data();
                entityManagerData.AssemblyQualifiedEntityTypeName = assemblyQualifiedEntityManagerEntityTypeName;
                entityManagerData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedEntityManagerEntityMetaDataTypeName;
                entityManagerData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedEntityManagerEntityDataTypeName;
                entityManagerData.ManagerName = "EntityManager";
                entityManagerData.ManagerParent = SystemManager.Instance;

                SetEntityMetaData(entityManagerMetaData);
                SetEntityData(entityManagerData);

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

        #region Methods
        public void RegisterEntity(Entity entity)
        {
            if (!registeredEntities.ContainsKey(entity.EntityID))
            {
                registeredEntities.Add(entity.EntityID, entity);
            }
        }

        public void UnregisterEntity(Entity entity)
        {
            if (registeredEntities.ContainsKey(entity.EntityID))
            {
                registeredEntities.Remove(entity.EntityID);
            }
        }

        public Entity GetEntity(Guid entityID)
        {
            if (registeredEntities.ContainsKey(entityID))
            {
                return registeredEntities[entityID];
            }
            return null;
        }

        public bool TryGetEntity(Guid entityID, out Entity entity)
        {
            if (!registeredEntities.ContainsKey(entityID))
            {
                entity = null;
                return false;
            }
            else
            {
                entity = registeredEntities[entityID];
                return true;
            }
        }

        public bool IsEntityRegistered(Guid entityID)
        {
            return registeredEntities.ContainsKey(entityID);
        }
        #endregion
    }
}
