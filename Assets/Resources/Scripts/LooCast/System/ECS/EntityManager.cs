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
                    instance = new EntityManager();
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
        private EntityManager() : base("EntityManager", SystemManager.Instance)
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
