using System;
using System.Collections.Generic;

namespace LooCast.System.ECS
{
    /// <summary>
    /// Lifecycle: Construction via Entity.AddComponent -> OnCreate -> SetMetaData -> SetData -> OnPreInitialize -> OnInitialize -> OnPostInitialize -> OnDestroy -> OnPreTerminate -> OnTerminate -> OnPostTerminate
    /// </summary>
    public abstract class Component : IComponent
    {
        #region Classes
        public class MetaData : IComponent.IMetaData
        {
            #region Properties
            public Guid ComponentID { get; set; }
            public string AssemblyQualifiedComponentTypeName { get; set; }
            public string AssemblyQualifiedComponentMetaDataTypeName { get; set; }
            public string AssemblyQualifiedComponentDataTypeName { get; set; }
            #endregion
        }

        public class Data : IComponent.IData
        {
            #region Properties
            public string AssemblyQualifiedComponentTypeName { get; set; }
            public string AssemblyQualifiedComponentMetaDataTypeName { get; set; }
            public string AssemblyQualifiedComponentDataTypeName { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public Type ComponentType { get; private set; }
        public Type ComponentMetaDataType { get; private set; }
        public Type ComponentDataType { get; private set; }
        public Guid ComponentID { get; private set; }
        public IEntity Entity { get; private set; }

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

        protected List<Action> preInitializationActions { get; private set; }
        protected List<Action> initializationActions { get; private set; }
        protected List<Action> postInitializationActions { get; private set; }

        protected List<Action> preTerminationActions { get; private set; }
        protected List<Action> terminationActions { get; private set; }
        protected List<Action> postTerminationActions { get; private set; }
        #endregion

        #region Constructors
        /// <summary>
        /// Component constructors are required be parameterless and should NEVER be called manually!
        /// </summary>
        protected Component()
        {
            IsCreated = false;
            IsDestroyed = false;
            HasMetaData = false;
            HasData = false;

            IsCreated_INTERNALLY = false;
            IsDestroyed_INTERNALLY = false;

            preInitializationActions = new List<Action>();
            initializationActions = new List<Action>();
            postInitializationActions = new List<Action>();

            preTerminationActions = new List<Action>();
            terminationActions = new List<Action>();
            postTerminationActions = new List<Action>();

            RegisterPostTerminationAction(() =>
            {
                ComponentManager.Instance.UnregisterComponent(this);
                ComponentID = Guid.Empty;
                Entity = null;
            });
        }
        #endregion

        #region Callbacks

        #region Initialization Phases
        /// <summary>
        /// Has to be manually called once after OnCreate!
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
        /// Has to be manually called once after OnPreInitialize!
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
        /// Has to be manually called once after OnInitialize!
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
        /// Automatically called when this component is being created. 
        /// Do NOT manually call this method! 
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnCreate()
        {
            IsCreated = true;
        }

        /// <summary>
        /// Automatically called when this component is destroyed. 
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
        public void Create_INTERNAL(Type componentType, Type componentMetaDataType, Type componentDataType, IEntity entity)
        {
            if (IsCreated_INTERNALLY)
            {
                throw new InvalidOperationException("Component has already been created internally!");
            }

            if (!typeof(IComponent).IsAssignableFrom(componentType))
            {
                throw new ArgumentException($"'{nameof(componentType)}' is not a component type!");
            }
            if (!typeof(IComponent.IMetaData).IsAssignableFrom(componentMetaDataType))
            {
                throw new ArgumentException($"'{nameof(componentMetaDataType)}' is not a component meta-data type!");
            }
            if (!typeof(IComponent.IData).IsAssignableFrom(componentDataType))
            {
                throw new ArgumentException($"'{nameof(componentDataType)}' is not a component data type!");
            }

            ComponentType = componentType;
            ComponentMetaDataType = componentMetaDataType;
            ComponentDataType = componentDataType;
            Entity = entity;

            IsCreated_INTERNALLY = true;
        }

        public void Destroy_INTERNAL()
        {
            if (IsDestroyed_INTERNALLY)
            {
                throw new InvalidOperationException("Component has already been destroyed internally!");
            }

            IsDestroyed_INTERNALLY = true;
        }

        #region Initialization Action Registration
        public void RegisterPreInitializationAction(Action preInitializationAction)
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

        public void RegisterInitializationAction(Action initializationAction)
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

        public void RegisterPostInitializationAction(Action postInitializationAction)
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
        public void RegisterPreTerminationAction(Action preTerminationAction)
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

        public void RegisterTerminationAction(Action terminationAction)
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

        public void RegisterPostTerminationAction(Action postTerminationAction)
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

        #region Data Management
        public virtual IComponent.IMetaData GetComponentMetaData()
        {
            if (!HasMetaData)
            {
                throw new InvalidOperationException($"Component '{this}' does not have metaData!");
            }

            IComponent.IMetaData componentMetaData = (IComponent.IMetaData)Activator.CreateInstance(ComponentMetaDataType);
            componentMetaData.AssemblyQualifiedComponentTypeName = ComponentType.AssemblyQualifiedName;
            componentMetaData.AssemblyQualifiedComponentMetaDataTypeName = ComponentMetaDataType.AssemblyQualifiedName;
            componentMetaData.AssemblyQualifiedComponentDataTypeName = ComponentDataType.AssemblyQualifiedName;
            componentMetaData.ComponentID = ComponentID;

            return componentMetaData;
        }

        public virtual IComponent.IData GetComponentData()
        {
            if (!HasData)
            {
                throw new InvalidOperationException($"Component '{this}' does not have data!");
            }

            IComponent.IData componentData = (IComponent.IData)Activator.CreateInstance(ComponentDataType);
            componentData.AssemblyQualifiedComponentTypeName = ComponentType.AssemblyQualifiedName;
            componentData.AssemblyQualifiedComponentMetaDataTypeName = ComponentMetaDataType.AssemblyQualifiedName;
            componentData.AssemblyQualifiedComponentDataTypeName = ComponentDataType.AssemblyQualifiedName;

            return componentData;
        }

        public virtual void SetComponentMetaData(IComponent.IMetaData componentMetaData)
        {
            ComponentID = componentMetaData.ComponentID;

            HasMetaData = true;
        }

        public virtual void SetComponentData(IComponent.IData componentData)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Cannot set data, because component '{this}' is not created!");
            }
            if (!HasMetaData)
            {
                throw new InvalidOperationException($"Cannot set data, because component '{this}' does not have metaData!");
            }

            HasData = true;
        }
        #endregion

        #endregion

        #region Overrides
        public override int GetHashCode()
        {
            return ComponentID.GetHashCode();
        }

        public override bool Equals(object obj)
        {
            if (obj is not Component)
            {
                return false;
            }

            Component other = (Component)obj;
            return other.ComponentID == this.ComponentID;
        }

        public override string ToString()
        {
            return $"Component[{ComponentID}]";
        }
        #endregion
    }
}
