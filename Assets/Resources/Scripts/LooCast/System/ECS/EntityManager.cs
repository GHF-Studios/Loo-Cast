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
                    string assemblyQualifiedEntityTypeName = typeof(EntityManager).AssemblyQualifiedName;
                    instance = Entity.Create<EntityManager>();

                    Entity.MetaData instanceMetaData = new Entity.MetaData
                        (
                            assemblyQualifiedEntityTypeName,
                            new Guid(),
                            new IComponent.IMetaData[]
                            {
                                new FolderComponent.MetaData(typeof(FolderComponent).AssemblyQualifiedName)
                            }
                        );

                    Manager.Data instanceData = new Manager.Data
                        (
                            assemblyQualifiedEntityTypeName,
                            new IComponent.IData[]
                            {
                                new FolderComponent.Data
                                    (
                                        typeof(FolderComponent).AssemblyQualifiedName,
                                        "EntityManager",
                                        SystemManager.Instance.GetComponent<FolderComponent>().FolderPath
                                    )
                            },
                            "EntityManager",
                            SystemManager.Instance
                        );


                    ((ISerializable<Entity.MetaData, Manager.Data>)instance).SetMetaData(instanceMetaData);
                    ((ISerializable<Entity.MetaData, Manager.Data>)instance).SetData(instanceData);
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
            RegisterPreSetupAction(() =>
            {
                registeredEntities = new Dictionary<Guid, IEntity>();
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
