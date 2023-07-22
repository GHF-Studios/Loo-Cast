using System;
using System.Linq;
using System.Reflection;
using System.Collections.Generic;

namespace LooCast.System.ECS
{
    using LooCast.System.Serialization;
    using UnityEngine.UIElements;

    /// <summary>
    /// Lifecycle: Construction -> OnCreate -> OnPreInitialize -> OnInitialize -> OnPostInitialize -> OnDestroy -> OnPreTerminate -> OnTerminate -> OnPostTerminate
    /// </summary>
    public abstract class Entity : IEntity, ISerializable<Entity.MetaData, Entity.Data>
    {
        #region Classes
        public class MetaData : IEntity.IMetaData
        {
            #region Properties
            public string AssemblyQualifiedEntityTypeName { get; set; }
            public Guid GUID { get; set; }
            public IComponent.IMetaData[] ComponentMetaDatas { get; set; }
            #endregion

            #region Constructors
            public MetaData(string assemblyQualifiedEntityTypeName)
            {
                AssemblyQualifiedEntityTypeName = assemblyQualifiedEntityTypeName;
                GUID = Guid.NewGuid();
                ComponentMetaDatas = Array.Empty<IComponent.IMetaData>();
            }

            public MetaData(string assemblyQualifiedEntityTypeName, Guid guid, IComponent.IMetaData[] componentMetaDatas)
            {
                AssemblyQualifiedEntityTypeName = assemblyQualifiedEntityTypeName;
                GUID = guid;
                ComponentMetaDatas = componentMetaDatas;
            }
            #endregion
        }

        public class Data : IEntity.IData
        {
            #region Properties
            public string AssemblyQualifiedEntityTypeName { get; set; }
            public IComponent.IData[] ComponentDatas { get; set; }
            #endregion

            #region Constructors
            public Data(string assemblyQualifiedEntityTypeName)
            {
                AssemblyQualifiedEntityTypeName = assemblyQualifiedEntityTypeName;
                ComponentDatas = Array.Empty<IComponent.IData>();
            }
            
            public Data(string assemblyQualifiedEntityTypeName, IComponent.IData[] componentDatas)
            {
                AssemblyQualifiedEntityTypeName = assemblyQualifiedEntityTypeName;
                ComponentDatas = componentDatas;
            }
            #endregion
        }
        #endregion

        #region Properties
        public Type EntityType { get; private set; }
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

            RegisterPreInitializationAction(() =>
            {
                EntityManager.Instance.RegisterEntity(this);
            });
            
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
        public static EntityType Create<EntityType>() where EntityType : IEntity, new()
        {
            Type newEntityType = typeof(EntityType);
            EntityType newEntity = new EntityType();
            newEntity.Create_INTERNAL(newEntityType);
            newEntity.OnCreate();
            return newEntity;
        }

        public static IEntity Create(Type entityType)
        {
            if (entityType == null)
            {
                throw new ArgumentNullException(nameof(entityType));
            }
            if (!typeof(IEntity).IsAssignableFrom(entityType))
            {
                throw new ArgumentException($"The given type {entityType} does not implement IEntity!");
            }

            IEntity newEntity = (IEntity)Activator.CreateInstance(entityType);
            newEntity.Create_INTERNAL(entityType);
            newEntity.OnCreate();
            return newEntity;
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
        /// Automatically called when this newEntity is being created. 
        /// Do NOT manually call this method! 
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnCreate()
        {
            IsCreated = true;
        }

        /// <summary>
        /// Automatically called when this newEntity is being destroyed. 
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
        public void Create_INTERNAL(Type entityType)
        {
            if (IsCreated_INTERNALLY)
            {
                throw new InvalidOperationException("Entity has already been created internally!");
            }
            
            EntityType = entityType;

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
        public ComponentType AddComponent<ComponentType>() where ComponentType : IComponent, new()
        {
            return (ComponentType)AddComponent(typeof(ComponentType));
        }

        public IComponent AddComponent(Type newComponentType)
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
            newComponent.Create_INTERNAL(newComponentType, this);
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
        Entity.MetaData ISerializable<Entity.MetaData, Entity.Data>.GetMetaData()
        {
            if (!HasMetaData)
            {
                throw new InvalidOperationException($"Entity '{this}' does not have metaData!");
            }

            IComponent.IMetaData[] componentMetaDatas;
            
            if (components.Count == 0)
            {
                componentMetaDatas = Array.Empty<IComponent.IMetaData>();
            }
            else
            {
                componentMetaDatas = new IComponent.IMetaData[components.Count];
                IComponent[] componentsArray = components.Values.ToArray();
                
                for (int i = 0; i < components.Count; i++)
                {
                    componentMetaDatas[i] = componentsArray[i].GetMetaData();
                }
            }
            
            return new Entity.MetaData(EntityType.AssemblyQualifiedName, EntityID, componentMetaDatas);
        }

        Entity.Data ISerializable<Entity.MetaData, Entity.Data>.GetData()
        {
            if (!HasData)
            {
                throw new InvalidOperationException($"Entity '{this}' does not have data!");
            }

            IComponent.IData[] componentDatas;
            
            if (components.Count == 0)
            {
                componentDatas = Array.Empty<IComponent.IData>();
            }
            else
            {
                componentDatas = new IComponent.IData[components.Count];
                IComponent[] componentsArray = components.Values.ToArray();

                for (int i = 0; i < components.Count; i++)
                {
                    componentDatas[i] = componentsArray[i].GetData();
                }
            }
            
            return new Entity.Data(EntityType.AssemblyQualifiedName, componentDatas);
        }

        void ISerializable<Entity.MetaData, Entity.Data>.SetMetaData(Entity.MetaData metaData)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Cannot set metaData, because entity '{this}' is not created!");
            }

            EntityID = metaData.GUID;

            for (int i = 0; i < metaData.ComponentMetaDatas.Length; i++)
            {
                IComponent.IMetaData newComponentMetaData = metaData.ComponentMetaDatas[i];
                Type newComponentType = Type.GetType(newComponentMetaData.AssemblyQualifiedComponentTypeName);
                IComponent newComponent = AddComponent(newComponentType);
                newComponent.SetMetaData(newComponentMetaData);
            }

            HasMetaData = true;
        }

        void ISerializable<Entity.MetaData, Entity.Data>.SetData(Entity.Data data)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Cannot set data, because entity '{this}' is not created!");
            }
            if (!HasMetaData)
            {
                throw new InvalidOperationException($"Cannot set data, because entity '{this}' does not have metaData!");
            }

            for (int i = 0; i < data.ComponentDatas.Length; i++)
            {
                IComponent.IData componentData = data.ComponentDatas[i];
                Type componentType = Type.GetType(componentData.AssemblyQualifiedComponentTypeName);
                components[componentType].SetData(componentData);
            }

            HasData = true;
        }

        IEntity.IMetaData ISerializable<IEntity.IMetaData, IEntity.IData>.GetMetaData()
        {
            return ((ISerializable<Entity.MetaData, Entity.Data>)this).GetMetaData();
        }

        IEntity.IData ISerializable<IEntity.IMetaData, IEntity.IData>.GetData()
        {
            return ((ISerializable<Entity.MetaData, Entity.Data>)this).GetData();
        }

        void ISerializable<IEntity.IMetaData, IEntity.IData>.SetMetaData(IEntity.IMetaData metaData)
        {
            if (metaData is not Entity.MetaData)
            {
                throw new ArgumentException($"MetaData '{nameof(metaData)}' is not of type 'Entity.MetaData'!");
            }

            ((ISerializable<Entity.MetaData, Entity.Data>)this).SetMetaData((Entity.MetaData)metaData);
        }

        void ISerializable<IEntity.IMetaData, IEntity.IData>.SetData(IEntity.IData data)
        {
            if (data is not Entity.Data)
            {
                throw new ArgumentException($"Data '{nameof(data)}' is not of type 'Entity.Data'!");
            }

            ((ISerializable<Entity.MetaData, Entity.Data>)this).SetData((Entity.Data)data);
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
            return EntityID.ToString();
        }
        #endregion
    }
}
