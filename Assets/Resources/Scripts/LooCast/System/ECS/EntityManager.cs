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
        private Dictionary<Guid, IEntity> registeredEntities;
        #endregion

        #region Constructors
        public EntityManager() : base()
        {
            registeredEntities = new Dictionary<Guid, IEntity>();

            FolderComponent folderComponent = AddComponent<FolderComponent, Component.MetaData, FolderComponent.Data>();

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

                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                string assemblyQualifiedFolderComponentTypeName = typeof(FolderComponent).AssemblyQualifiedName;
                string assemblyQualifiedFolderComponentMetaDataTypeName = typeof(Component.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedFolderComponentDataTypeName = typeof(FolderComponent.Data).AssemblyQualifiedName;

                Component.MetaData folderComponentMetaData = new Component.MetaData();
                folderComponentMetaData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                folderComponentMetaData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                folderComponentMetaData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                folderComponentMetaData.ComponentID = new Guid();

                FolderComponent.Data folderComponentData = new FolderComponent.Data();
                folderComponentData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                folderComponentData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                folderComponentData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                folderComponentData.FolderName = "EntityManager";
                folderComponentData.ParentFolderPath = SystemManager.Instance.GetComponent<FolderComponent>().FolderPath;

                folderComponent.SetComponentMetaData(folderComponentMetaData);
                folderComponent.SetComponentData(folderComponentData);

                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnSetup();
                }

                FolderManager.Instance.RegisterFolder(folderComponent);
            });

            RegisterPostSetupAction(() =>
            {
                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPostSetup();
                }
            });

            RegisterPreInitializationAction(() =>
            {
                folderComponent.OnPreInitialize();
            });

            RegisterInitializationAction(() =>
            {
                folderComponent.OnInitialize();
            });

            RegisterPostInitializationAction(() =>
            {
                folderComponent.OnPostInitialize();
            });
        }
        #endregion

        #region Methods
        public void RegisterEntity(IEntity entity)
        {
            if (!registeredEntities.ContainsKey(entity.EntityID))
            {
                registeredEntities.Add(entity.EntityID, entity);
            }
        }

        public void UnregisterEntity(IEntity entity)
        {
            if (registeredEntities.ContainsKey(entity.EntityID))
            {
                registeredEntities.Remove(entity.EntityID);
            }
        }

        public IEntity GetEntity(Guid entityID)
        {
            if (registeredEntities.ContainsKey(entityID))
            {
                return registeredEntities[entityID];
            }
            return null;
        }

        public bool TryGetEntity(Guid entityID, out IEntity entity)
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
