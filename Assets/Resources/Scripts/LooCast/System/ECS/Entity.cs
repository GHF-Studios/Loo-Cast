using System;
using System.Reflection;
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
        private Dictionary<Guid, Type> componentTypes;
        #endregion
        
        #region Constructors
        protected Entity()
        {
            EntityID = Guid.NewGuid();
            components = new Dictionary<Type, IComponent>();
            componentTypes = new Dictionary<Guid, Type>();
            OnCreate();
        }
        #endregion

        #region Finalizers
        ~Entity()
        {
            OnDestroy();
        }
        #endregion
        
        #region Static Methods
        public static void Destroy(Entity entity)
        {
            if (entity == null)
            {
                throw new ArgumentNullException(nameof(entity));
            }

            entity.OnDestroy();
        }
        #endregion

        #region Callbacks
        protected virtual void OnCreate()
        {
            EntityManager.Instance.RegisterEntity(this);
        }

        protected virtual void OnDestroy()
        {
            if (IsUnityBridgeEnabled)
            {
                DisableUnityBridge();
            }

            foreach (Type componentType in componentTypes.Values)
            {
                RemoveComponent(componentType);
            }
            components.Clear();
            componentTypes.Clear();

            EntityManager.Instance.UnregisterEntity(this);
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
            Type newComponentType = typeof(ComponentType);
            ComponentType newComponent = new ComponentType();

            if (components.ContainsKey(newComponentType))
            {
                throw new InvalidOperationException($"Entity '{this}' already contains a component of type '{typeof(ComponentType).Name}'!");
            }

            foreach (Type componentType in components.Keys)
            {
                IncompatibleComponentsAttribute incompatibleComponentsAttribute = componentType.GetCustomAttribute<IncompatibleComponentsAttribute>();
                if (incompatibleComponentsAttribute != null)
                {
                    foreach (Type incompatibleComponentType in incompatibleComponentsAttribute.IncompatibleComponentTypes)
                    {
                        if (newComponentType == incompatibleComponentType)
                        {
                            throw new InvalidOperationException($"Cannot add component of type '{newComponentType.Name}', as it is incompatible with already added component of type '{componentType.Name}'!");
                        }
                    }
                }
            }

            components.Add(newComponentType, newComponent);
            componentTypes.Add(newComponent.ComponentID, newComponentType);
            newComponent.Initialize_INTERNAL(this);
            newComponent.OnCreate();

            return newComponent;
        }

        public void RemoveComponent(IComponent component)
        {
            RemoveComponent(component.ComponentID);
        }

        public void RemoveComponent(Guid componentID)
        {
            if (!componentTypes.ContainsKey(componentID))
            {
                throw new InvalidOperationException($"Entity '{this}' does not contain a component with ID '{componentID}'!");
            }

            Type componentType = componentTypes[componentID];
            RemoveComponent(componentType);
        }

        public void RemoveComponent<ComponentType>() where ComponentType : IComponent, new()
        {
            Type componentType = typeof(ComponentType);
            RemoveComponent(componentType);
        }

        public void RemoveComponent(Type componentType)
        {
            if (!components.ContainsKey(componentType))
            {
                throw new InvalidOperationException($"Entity '{this}' does not contain a component of type '{componentType.Name}'!");
            }

            IComponent component = components[componentType];
            component.OnDestroy();
            component.Destroy_INTERNAL();
            components.Remove(componentType);
            componentTypes.Remove(component.ComponentID);
        }

        public bool ContainsComponent<ComponentType>() where ComponentType : IComponent, new()
        {
            return components.ContainsKey(typeof(ComponentType));
        }

        public bool ContainsComponent(Type componentType)
        {
            return components.ContainsKey(componentType);
        }
        
        public ComponentType GetComponent<ComponentType>() where ComponentType : IComponent, new()
        {
            if (!components.TryGetValue(typeof(ComponentType), out IComponent component))
            {
                return default;
            }
            return (ComponentType)component;
        }

        public IComponent GetComponent(Type componentType)
        {
            if (!components.TryGetValue(componentType, out IComponent component))
            {
                return default;
            }
            return component;
        }

        public bool TryGetComponent<ComponentType>(out IComponent component) where ComponentType : IComponent, new()
        {
            return components.TryGetValue(typeof(ComponentType), out component);
        }

        public bool TryGetComponent(Type componentType, out IComponent component)
        {
            return components.TryGetValue(componentType, out component);
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
