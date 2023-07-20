using System;
using System.Collections.Generic;

namespace LooCast.System.ECS
{
    public abstract class Entity : IEntity
    {
        #region Properties
        public Guid EntityID { get; private set; }
        public UnityBridge UnityBridge { get; private set; }
        public bool IsUnityBridgeEnabled => UnityBridge != null;
        #endregion

        #region Fields
        private Dictionary<Type, IComponent> components;
        #endregion
        
        #region Constructors
        protected Entity()
        {
            EntityID = Guid.NewGuid();
            components = new Dictionary<Type, IComponent>();
        }
        #endregion

        #region Methods
        public virtual void EnableUnityBridge()
        {
            if (IsUnityBridgeEnabled)
            {
                throw new Exception("UnityBridge is already enabled!");
            }
            
            UnityBridge = new UnityBridge();
            UnityBridge.RootGameObject.name = "New Entity";
        }

        public virtual void DisableUnityBridge()
        {
            if (!IsUnityBridgeEnabled)
            {
                throw new Exception("UnityBridge is already disabled!");
            }
            
            UnityBridge.Terminate();
            UnityBridge = null;
        }

        public ComponentType AddComponent<ComponentType>() where ComponentType : IComponent, new()
        {
            Type componentType = typeof(ComponentType);
            ComponentType component = new ComponentType();

            if (components.ContainsKey(componentType))
            {
                throw new InvalidOperationException($"Entity '{this}' already contains a component of type '{typeof(ComponentType).Name}'!");
            }

            components.Add(componentType, component);
            component.Initialize_INTERNAL(this);
            component.OnCreate();

            return component;
        }

        public void RemoveComponent<ComponentType>() where ComponentType : IComponent, new()
        {
            Type componentType = typeof(ComponentType);

            if (!components.ContainsKey(componentType))
            {
                throw new InvalidOperationException($"Entity '{this}' does not contain a component of type '{typeof(ComponentType).Name}'!");
            }

            IComponent component = components[componentType];
            component.OnDestroy();
            component.Destroy_INTERNAL();
            components.Remove(componentType);
        }

        public bool ContainsComponent<ComponentType>() where ComponentType : IComponent, new()
        {
            return components.ContainsKey(typeof(ComponentType));
        }
        
        public ComponentType GetComponent<ComponentType>() where ComponentType : IComponent, new()
        {
            if (!components.TryGetValue(typeof(ComponentType), out IComponent component))
            {
                return default;
            }
            return (ComponentType)component;
        }

        public bool TryGetComponent<ComponentType>(out IComponent component) where ComponentType : IComponent, new()
        {
            return components.TryGetValue(typeof(ComponentType), out component);
        }
        #endregion

        #region Overrides
        public override int GetHashCode()
        {
            return EntityID.GetHashCode();
        }

        public override bool Equals(object obj)
        {
            if (obj is not Entity)
            {
                return false;
            }

            Entity other = (Entity)obj;
            return other.EntityID == this.EntityID;
        }

        public override string ToString()
        {
            return EntityID.ToString();
        }
        #endregion
    }
}
