using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.ECS
{
    using LooCast.System.Serialization;
    using LooCast.System.ECS;
    using LooCast.Core;

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

                    string assemblyQualifiedMainManagerEntityTypeName = typeof(EntityManager).AssemblyQualifiedName;
                    string assemblyQualifiedMainManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                    string assemblyQualifiedMainManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                    string assemblyQualifiedFolderComponentTypeName = typeof(FolderComponent).AssemblyQualifiedName;
                    string assemblyQualifiedFolderComponentMetaDataTypeName = typeof(Component.MetaData).AssemblyQualifiedName;
                    string assemblyQualifiedFolderComponentDataTypeName = typeof(FolderComponent.Data).AssemblyQualifiedName;

                    Entity.MetaData instanceMetaData = new Entity.MetaData();
                    instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedMainManagerEntityTypeName;
                    instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedMainManagerEntityMetaDataTypeName;
                    instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedMainManagerEntityDataTypeName;
                    instanceMetaData.GUID = new Guid();
                    IFolderComponent.IMetaData folderComponentMetaData = new FolderComponent.MetaData();
                    folderComponentMetaData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                    folderComponentMetaData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                    folderComponentMetaData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                    folderComponentMetaData.GUID = new Guid();
                    instanceMetaData.ComponentMetaDatas = new IComponent.IMetaData[]
                    {
                        folderComponentMetaData
                    };

                    Manager.Data instanceData = new Manager.Data();
                    instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedMainManagerEntityTypeName;
                    instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedMainManagerEntityMetaDataTypeName;
                    instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedMainManagerEntityDataTypeName;
                    IFolderComponent.IData folderComponentData = new FolderComponent.Data();
                    folderComponentData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                    folderComponentData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                    folderComponentData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                    folderComponentData.FolderName = "EntityManager";
                    folderComponentData.ParentFolderPath = SystemManager.Instance.GetComponent<FolderComponent>().FolderPath;
                    instanceData.ComponentDatas = new IComponent.IData[]
                    {
                        folderComponentData
                    };
                    instanceData.ManagerName = "EntityManager";
                    instanceData.ManagerParent = SystemManager.Instance;

                    instance.SetMetaData(instanceMetaData);
                    instance.SetData(instanceData);
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
