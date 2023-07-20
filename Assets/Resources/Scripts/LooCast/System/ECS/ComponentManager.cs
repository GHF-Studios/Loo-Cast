using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.ECS
{
    public sealed class ComponentManager : ModuleManager
    {
        #region Static Properties
        public static ComponentManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new ComponentManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static ComponentManager instance;
        #endregion

        #region Fields
        private Dictionary<Guid, IComponent> registeredEntities;
        #endregion

        #region Constructors
        private ComponentManager() : base("ComponentManager", SystemManager.Instance)
        {
            registeredEntities = new Dictionary<Guid, IComponent>();
        }
        #endregion

        #region Methods
        public void RegisterComponent(IComponent component)
        {
            if (!registeredEntities.ContainsKey(component.ComponentID))
            {
                registeredEntities.Add(component.ComponentID, component);
            }

            if (enableLogging)
            {
                Debug.LogWarning($"Registered Component '{component}'.");
            }
        }

        public void UnregisterComponent(IComponent component)
        {
            if (registeredEntities.ContainsKey(component.ComponentID))
            {
                registeredEntities.Remove(component.ComponentID);
            }
        }

        public IComponent GetComponent(Guid componentID)
        {
            if (registeredEntities.ContainsKey(componentID))
            {
                return registeredEntities[componentID];
            }
            return null;
        }

        public bool TryGetComponent(Guid componentID, out IComponent component)
        {
            if (!registeredEntities.ContainsKey(componentID))
            {
                component = null;
                return false;
            }
            else
            {
                component = registeredEntities[componentID];
                return true;
            }
        }

        public bool IsComponentRegistered(Guid componentID)
        {
            return registeredEntities.ContainsKey(componentID);
        }
        #endregion
    }
}
