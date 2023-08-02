using System;
using System.Linq;
using System.Reflection;
using System.Collections.Generic;
using UnityEngine;
using System.Xml.Linq;

namespace LooCast.System.ECS
{
    /// <summary>
    /// Lifecycle: Construction via Entity.Create -> OnCreate -> SetMetaData -> SetData -> OnPreInitialize -> OnInitialize -> OnPostInitialize -> OnDestroy -> OnPreTerminate -> OnTerminate -> OnPostTerminate
    /// </summary>
    public abstract class Entity : IEntity
    {
        #region Classes
        public class MetaData : IEntity.IMetaData
        {
            #region Properties
            public Guid EntityID { get; set; }
            public string AssemblyQualifiedEntityTypeName { get; set; }
            public string AssemblyQualifiedEntityMetaDataTypeName { get; set; }
            public string AssemblyQualifiedEntityDataTypeName { get; set; }

            #endregion

            #region Methods
            public void Serialize(string objectName, out XElement serializedObject)
            {
                serializedObject = new XElement(objectName);
                serializedObject.Add(new XAttribute(nameof(EntityID), EntityID));
            }

            public void Deserialize(XElement serializedObject)
            {
                throw new NotImplementedException();
            }
            #endregion
        }

        public class Data : IEntity.IData
        {
            #region Properties
            public string AssemblyQualifiedEntityTypeName { get; set; }
            public string AssemblyQualifiedEntityMetaDataTypeName { get; set; }
            public string AssemblyQualifiedEntityDataTypeName { get; set; }
            #endregion
        }

        public sealed class FullMetaData : IEntity.IFullMetaData
        {
            #region Properties
            public IEntity.IMetaData EntityMetaData { get; set; }
            public IComponent.IMetaData[] ComponentMetaDatas { get; set; }
            #endregion
        }

        public sealed class FullData : IEntity.IFullData
        {
            #region Properties
            public IEntity.IData EntityData { get; set; }
            public IComponent.IData[] ComponentDatas { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public Type EntityType { get; private set; }
        public Type EntityMetaDataType { get; private set; }
        public Type EntityDataType { get; private set; }
        public Guid EntityID { get; private set; }
        public UnityBridge UnityBridge { get; private set; }
        public bool IsUnityBridgeEnabled => UnityBridge != null;

        public bool IsPreInitializing { get; protected set; }
        public bool IsPreInitialized { get; protected set; }
        public bool IsInitializing { get; protected set; }
        public bool IsInitialized { get; protected set; }
        public bool IsPostInitializing { get; protected set; }
        public bool IsPostInitialized { get; protected set; }

        public bool IsPreTerminating { get; protected set; }
        public bool IsPreTerminated { get; protected set; }
        public bool IsTerminating { get; protected set; }
        public bool IsTerminated { get; protected set; }
        public bool IsPostTerminating { get; protected set; }
        public bool IsPostTerminated { get; protected set; }

        public bool IsCreated { get; protected set; }
        public bool IsDestroyed { get; protected set; }
        public bool HasMetaData { get; protected set; }
        public bool HasData { get; protected set; }
        
        public bool IsCreated_INTERNALLY { get; private set; }
        public bool IsDestroyed_INTERNALLY { get; private set; }

        protected List<Action> preInitializationActions { get; private set;}
        protected List<Action> initializationActions { get; private set;}
        protected List<Action> postInitializationActions { get; private set;}
        
        protected List<Action> preTerminationActions { get; private set;}
        protected List<Action> terminationActions { get; private set;}
        protected List<Action> postTerminationActions { get; private set; }
        #endregion

        #region Fields
        private Dictionary<Type, IComponent> components;
        private Dictionary<Guid, Type> componentTypes;
        #endregion

        #region Constructors
        /// <summary>
        /// Entity constructors are required be parameterless and should NEVER be called manually!
        /// </summary>
        protected Entity()
        {
            IsCreated = false;
            IsDestroyed = false;
            HasMetaData = false;
            HasData = false;

            IsCreated_INTERNALLY = false;
            IsDestroyed_INTERNALLY = false;

            components = new Dictionary<Type, IComponent>();
            componentTypes = new Dictionary<Guid, Type>();

            preInitializationActions = new List<Action>();
            initializationActions = new List<Action>();
            postInitializationActions = new List<Action>();

            preTerminationActions = new List<Action>();
            terminationActions = new List<Action>();
            postTerminationActions = new List<Action>();
            
            RegisterPostTerminationAction(() =>
            {
                EntityManager.Instance.UnregisterEntity(this);

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
            });
        }
        #endregion
        
        #region Static Methods
        public static EntityType Create<EntityType, EntityMetaDataType, EntityDataType>() 
            where EntityType : IEntity, new()
            where EntityMetaDataType : IEntity.IMetaData, new()
            where EntityDataType : IEntity.IData, new()
        {
            return (EntityType)Create(typeof(EntityType), typeof(EntityMetaDataType), typeof(EntityDataType));
        }

        public static IEntity Create(Type entityType, Type entityMetaDataType, Type entityDataType)
        {
            if (entityType == null)
            {
                throw new ArgumentNullException(nameof(entityType));
            }
            if (entityMetaDataType == null)
            {
                throw new ArgumentNullException(nameof(entityMetaDataType));
            }
            if (entityDataType == null)
            {
                throw new ArgumentNullException(nameof(entityDataType));
            }
            if (!typeof(IEntity).IsAssignableFrom(entityType))
            {
                throw new ArgumentException($"The given type {entityType} does not implement IEntity!");
            }
            if (!typeof(IEntity.IMetaData).IsAssignableFrom(entityMetaDataType))
            {
                throw new ArgumentException($"The given type {entityMetaDataType} does not implement IEntity.IMetaData!");
            }
            if (!typeof(IEntity.IData).IsAssignableFrom(entityDataType))
            {
                throw new ArgumentException($"The given type {entityDataType} does not implement IEntity.IData!");
            }

            IEntity entity = (IEntity)Activator.CreateInstance(entityType);
            entity.Create_INTERNAL(entityType, entityMetaDataType, entityDataType);
            entity.OnCreate();
            return entity;
        }

        public static IEntity Create(IEntity.IFullMetaData fullEntityMetaData)
        {
            if (fullEntityMetaData == null)
            {
                throw new ArgumentNullException(nameof(fullEntityMetaData));
            }

            Type entityType = Type.GetType(fullEntityMetaData.EntityMetaData.AssemblyQualifiedEntityTypeName);
            Type entityMetaDataType = Type.GetType(fullEntityMetaData.EntityMetaData.AssemblyQualifiedEntityMetaDataTypeName);
            Type entityDataType = Type.GetType(fullEntityMetaData.EntityMetaData.AssemblyQualifiedEntityDataTypeName);

            IEntity entity = Create(entityType, entityMetaDataType, entityDataType);

            foreach (IComponent.IMetaData componentMetaData in fullEntityMetaData.ComponentMetaDatas)
            {
                Type componentType = Type.GetType(componentMetaData.AssemblyQualifiedComponentTypeName);
                Type componentMetaDataType = Type.GetType(componentMetaData.AssemblyQualifiedComponentMetaDataTypeName);
                Type componentDataType = Type.GetType(componentMetaData.AssemblyQualifiedComponentDataTypeName);

                entity.AddComponent(componentType, componentMetaDataType, componentDataType);
            }

            return entity;
        }

        /// <summary>
        /// Do NOT use this to destroy the MainManager! Instead invoke LooCastApplication.Exit()!
        /// </summary>
        public static void Destroy(IEntity entity)
        {
            if (entity == null)
            {
                throw new ArgumentNullException(nameof(entity));
            }
            if (entity.Equals(MainManager.Instance))
            {
                throw new InvalidOperationException($"The MainManager can not be deleted via Entity.Destroy! If you tried to exit the application, use LooCastApplication.Exit.");
            }

            entity.Destroy_INTERNAL();
            entity.OnDestroy();
        }
        #endregion

        #region Callbacks

        #region Initialization Phases
        /// <summary>
        /// Has to be manually called once after OnCreate.
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnPreInitialize()
        {
            if (!HasData)
            {
                throw new InvalidOperationException("Cannot pre-initialize, because the data has not been set!");
            }
            if (IsPreInitializing)
            {
                throw new InvalidOperationException("Cannot pre-initialize while already pre-initializing!");
            }
            if (IsPreInitialized)
            {
                throw new InvalidOperationException("Cannot pre-initialize while already pre-initialized!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot pre-initialize while already initializing!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot pre-initialize while already initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot pre-initialize while already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot pre-initialize while already post-initialized!");
            }

            IsPreInitializing = true;

            foreach (Action preInitializationAction in preInitializationActions)
            {
                preInitializationAction.Invoke();
            }

            IsPreInitializing = false;
            IsPreInitialized = true;
        }

        /// <summary>
        /// Has to be manually called once after OnPreInitialize.
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnInitialize()
        {
            if (!HasData)
            {
                throw new InvalidOperationException("Cannot initialize, because the data has not been set!");
            }
            if (IsPreInitializing)
            {
                throw new InvalidOperationException("Cannot initialize while already pre-initializing!");
            }
            if (!IsPreInitialized)
            {
                throw new InvalidOperationException("Cannot initialize while not pre-initialized!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot initialize while already initializing!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot initialize while already initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot initialize while already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot initialize while already post-initialized!");
            }

            IsInitializing = true;

            foreach (Action initializationAction in initializationActions)
            {
                initializationAction.Invoke();
            }

            IsInitializing = false;
            IsInitialized = true;
        }

        /// <summary>
        /// Has to be manually called once after OnInitialize.
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnPostInitialize()
        {
            if (!HasData)
            {
                throw new InvalidOperationException("Cannot post-initialize, because the data has not been set!");
            }
            if (IsPreInitializing)
            {
                throw new InvalidOperationException("Cannot post-initialize while already pre-initializing!");
            }
            if (!IsPreInitialized)
            {
                throw new InvalidOperationException("Cannot post-initialize while not pre-initialized!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot post-initialize while already initializing!");
            }
            if (!IsInitialized)
            {
                throw new InvalidOperationException("Cannot post-initialize while not initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot post-initialize while already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot post-initialize while already post-initialized!");
            }

            IsPostInitializing = true;

            foreach (Action postInitializationAction in postInitializationActions)
            {
                postInitializationAction.Invoke();
            }

            IsPostInitializing = false;
            IsPostInitialized = true;
        }
        #endregion

        #region Termination Phases
        /// <summary>
        /// Automatically called after OnDestroy. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnPreTerminate()
        {
            if (IsPreTerminating)
            {
                throw new InvalidOperationException("Cannot pre-terminate while already pre-terminating!");
            }
            if (IsPreTerminated)
            {
                throw new InvalidOperationException("Cannot pre-terminate while already pre-terminated!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot pre-terminate while already terminating!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot pre-terminate while already terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot pre-terminate while already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot pre-terminate while already post-terminated!");
            }

            IsPreTerminating = true;

            foreach (Action preTerminationAction in preTerminationActions)
            {
                preTerminationAction.Invoke();
            }

            IsPreTerminating = false;
            IsPreTerminated = true;
        }

        /// <summary>
        /// Automatically called after OnPreTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnTerminate()
        {
            if (IsPreTerminating)
            {
                throw new InvalidOperationException("Cannot terminate while already pre-terminating!");
            }
            if (!IsPreTerminated)
            {
                throw new InvalidOperationException("Cannot terminate while not pre-terminated!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot terminate while already terminating!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot terminate while already terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot terminate while already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot terminate while already post-terminated!");
            }

            IsTerminating = true;

            foreach (Action terminationAction in terminationActions)
            {
                terminationAction.Invoke();
            }

            IsTerminating = false;
            IsTerminated = true;
        }

        /// <summary>
        /// Automatically called after OnTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnPostTerminate()
        {
            if (IsPreTerminating)
            {
                throw new InvalidOperationException("Cannot post-terminate while already pre-terminating!");
            }
            if (!IsPreTerminated)
            {
                throw new InvalidOperationException("Cannot post-terminate while not pre-terminated!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot post-terminate while already terminating!");
            }
            if (!IsTerminated)
            {
                throw new InvalidOperationException("Cannot post-terminate while not terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot post-terminate while already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot post-terminate while already post-terminated!");
            }

            IsPostTerminating = true;

            foreach (Action postTerminationAction in postTerminationActions)
            {
                postTerminationAction.Invoke();
            }

            IsPostTerminating = false;
            IsPostTerminated = true;
        }
        #endregion
        
        /// <summary>
        /// Automatically called when this entity is being created. 
        /// Do NOT manually call this method! 
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnCreate()
        {
            IsCreated = true;
        }

        /// <summary>
        /// Automatically called when this entity is being destroyed. 
        /// Do NOT manually call this method! 
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnDestroy()
        {
            IsDestroyed = true;
            OnPreTerminate();
            OnTerminate();
            OnPostTerminate();
        }
        #endregion

        #region Methods
        public void Create_INTERNAL(Type entityType, Type entityMetaDataType, Type entityDataType)
        {
            if (IsCreated_INTERNALLY)
            {
                throw new InvalidOperationException("Entity has already been created internally!");
            }

            if (!typeof(IEntity).IsAssignableFrom(entityType))
            {
                throw new ArgumentException($"'{nameof(entityType)}' is not an entity type!");
            }
            if (!typeof(IEntity.IMetaData).IsAssignableFrom(entityMetaDataType))
            {
                throw new ArgumentException($"'{nameof(entityMetaDataType)}' is not an entity metadata type!");
            }
            if (!typeof(IEntity.IData).IsAssignableFrom(entityDataType))
            {
                throw new ArgumentException($"'{nameof(entityDataType)}' is not an entity data type!");
            }

            EntityType = entityType;
            EntityMetaDataType = entityMetaDataType;
            EntityDataType = entityDataType;

            IsCreated_INTERNALLY = true;
        }

        public void Destroy_INTERNAL()
        {
            if (IsDestroyed_INTERNALLY)
            {
                throw new InvalidOperationException("Entity has already been destroyed internally!");
            }
            
            IsDestroyed_INTERNALLY = true;
        }

        #region Initialization Action Registration
        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterPreInitializationAction(Action preInitializationAction)
        {
            if (IsPreInitializing)
            {
                throw new InvalidOperationException("Cannot register pre-initialization action while already pre-initializing!");
            }
            if (IsPreInitialized)
            {
                throw new InvalidOperationException("Cannot register pre-initialization action while already pre-initialized!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot register pre-initialization action while already initializing!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot register pre-initialization action while already initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register pre-initialization action while already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register pre-initialization action while already post-initialized!");
            }

            preInitializationActions.Add(preInitializationAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterInitializationAction(Action initializationAction)
        {
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot register initialization action while already initializing!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot register initialization action while already initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register initialization action while already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register initialization action while already post-initialized!");
            }

            initializationActions.Add(initializationAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterPostInitializationAction(Action postInitializationAction)
        {
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register post-initialization action while already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register post-initialization action while already post-initialized!");
            }
            
            postInitializationActions.Add(postInitializationAction);
        }
        #endregion

        #region Termination Action Registration
        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterPreTerminationAction(Action preTerminationAction)
        {
            if (IsPreTerminating)
            {
                throw new InvalidOperationException("Cannot register pre-termination action while already pre-terminating!");
            }
            if (IsPreTerminated)
            {
                throw new InvalidOperationException("Cannot register pre-termination action while already pre-terminated!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot register pre-termination action while already terminating!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot register pre-termination action while already terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register pre-termination action while already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register pre-termination action while already post-terminated!");
            }

            preTerminationActions.Add(preTerminationAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterTerminationAction(Action terminationAction)
        {
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot register termination action while already terminating!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot register termination action while already terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register termination action while already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register termination action while already post-terminated!");
            }

            terminationActions.Add(terminationAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterPostTerminationAction(Action postTerminationAction)
        {
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register post-termination action while already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register post-termination action while already post-terminated!");
            }

            postTerminationActions.Add(postTerminationAction);
        }
        #endregion

        #region Unity Bridge Management
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
        #endregion

        #region Component Management
        public ComponentType AddComponent<ComponentType, ComponentMetaDataType, ComponentDataType>() 
            where ComponentType : IComponent, new()
            where ComponentMetaDataType : IComponent.IMetaData, new()
            where ComponentDataType : IComponent.IData, new()
        {
            return (ComponentType)AddComponent(typeof(ComponentType), typeof(ComponentMetaDataType), typeof(ComponentDataType));
        }
        
        public IComponent AddComponent(Type newComponentType, Type newComponentMetaDataType, Type newComponentDataType)
        {
            IComponent newComponent = (IComponent)Activator.CreateInstance(newComponentType);

            if (components.ContainsKey(newComponentType))
            {
                throw new InvalidOperationException($"Entity '{this}' already contains a component of type '{newComponentType.Name}'!");
            }

            foreach (Type alreadyAddedComponentType in components.Keys)
            {
                IncompatibleComponentsAttribute incompatibleComponentsAttribute = alreadyAddedComponentType.GetCustomAttribute<IncompatibleComponentsAttribute>();
                if (incompatibleComponentsAttribute != null)
                {
                    foreach (Type incompatibleComponentType in incompatibleComponentsAttribute.IncompatibleComponentTypes)
                    {
                        if (newComponentType == incompatibleComponentType)
                        {
                            throw new InvalidOperationException($"Cannot add component of type '{newComponentType.Name}', as it is incompatible with already added component of type '{alreadyAddedComponentType.Name}'!");
                        }
                    }
                }
            }

            components.Add(newComponentType, newComponent);
            componentTypes.Add(newComponent.ComponentID, newComponentType);
            newComponent.Create_INTERNAL(newComponentType, newComponentMetaDataType, newComponentDataType, this);
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
            component.Destroy_INTERNAL();
            component.OnDestroy();
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
        
        #region Data Management
        public virtual IEntity.IMetaData GetEntityMetaData()
        {
            if (!HasMetaData)
            {
                throw new InvalidOperationException($"Entity '{this}' does not have metaData!");
            }

            IEntity.IMetaData entityMetaData = (IEntity.IMetaData)Activator.CreateInstance(EntityMetaDataType);
            entityMetaData.AssemblyQualifiedEntityTypeName = EntityType.AssemblyQualifiedName;
            entityMetaData.AssemblyQualifiedEntityMetaDataTypeName = EntityMetaDataType.AssemblyQualifiedName;
            entityMetaData.AssemblyQualifiedEntityDataTypeName = EntityDataType.AssemblyQualifiedName;
            entityMetaData.EntityID = EntityID;

            return entityMetaData;
        }

        public virtual IEntity.IData GetEntityData()
        {
            if (!HasData)
            {
                throw new InvalidOperationException($"Entity '{this}' does not have data!");
            }

            IEntity.IData entityData = (IEntity.IData)Activator.CreateInstance(EntityDataType);
            entityData.AssemblyQualifiedEntityTypeName = EntityType.AssemblyQualifiedName;
            entityData.AssemblyQualifiedEntityMetaDataTypeName = EntityMetaDataType.AssemblyQualifiedName;
            entityData.AssemblyQualifiedEntityDataTypeName = EntityDataType.AssemblyQualifiedName;
            
            return entityData;
        }

        public virtual void SetEntityMetaData(IEntity.IMetaData entityMetaData)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Cannot set metaData, because entity '{this}' is not created!");
            }

            EntityID = entityMetaData.EntityID;

            HasMetaData = true;
        }

        public virtual void SetEntityData(IEntity.IData entityData)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Cannot set data, because entity '{this}' is not created!");
            }
            if (!HasMetaData)
            {
                throw new InvalidOperationException($"Cannot set data, because entity '{this}' does not have metaData!");
            }

            HasData = true;
        }

        public IEntity.IFullMetaData GetFullEntityMetaData()
        {
            IEntity.IFullMetaData fullEntityMetaData = new FullMetaData();
            fullEntityMetaData.EntityMetaData = GetEntityMetaData();
            IComponent.IMetaData[] componentMetaDatas;
            if (components.Count == 0)
            {
                componentMetaDatas = Array.Empty<IComponent.IMetaData>();
            }
            else
            {
                IComponent[] entityComponents = components.Values.ToArray();
                componentMetaDatas = new IComponent.IMetaData[entityComponents.Length];
                for (int i = 0; i < entityComponents.Length; i++)
                {
                    componentMetaDatas[i] = entityComponents[i].GetComponentMetaData();
                }
            }
            fullEntityMetaData.ComponentMetaDatas = componentMetaDatas;
            return fullEntityMetaData;
        }

        public IEntity.IFullData GetFullEntityData()
        {
            IEntity.IFullData fullEntityData = new FullData();
            fullEntityData.EntityData = GetEntityData();
            IComponent.IData[] componentDatas;
            if (components.Count == 0)
            {
                componentDatas = Array.Empty<IComponent.IData>();
            }
            else
            {
                IComponent[] entityComponents = components.Values.ToArray();
                componentDatas = new IComponent.IData[entityComponents.Length];
                for (int i = 0; i < entityComponents.Length; i++)
                {
                    componentDatas[i] = entityComponents[i].GetComponentData();
                }
            }
            fullEntityData.ComponentDatas = componentDatas;
            return fullEntityData;
        }
        
        public void SetFullEntityMetaData(IEntity.IFullMetaData fullEntityMetaData)
        {
            SetEntityMetaData(fullEntityMetaData.EntityMetaData);
            
            foreach (IComponent.IMetaData componentMetaData in fullEntityMetaData.ComponentMetaDatas)
            {
                Type componentType = Type.GetType(componentMetaData.AssemblyQualifiedComponentTypeName);
                components[componentType].SetComponentMetaData(componentMetaData);
            }
        }

        public void SetFullEntityData(IEntity.IFullData fullEntityData)
        {
            SetEntityData(fullEntityData.EntityData);

            foreach (IComponent.IData componentData in fullEntityData.ComponentDatas)
            {
                Type componentType = Type.GetType(componentData.AssemblyQualifiedComponentTypeName);
                components[componentType].SetComponentData(componentData);
            }
        }
        #endregion

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
            return $"Component[{EntityID}]";
        }
        #endregion
    }
}
