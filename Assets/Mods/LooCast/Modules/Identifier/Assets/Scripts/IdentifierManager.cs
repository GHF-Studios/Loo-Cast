using LooCast.Core;
using System;
using System.Reflection;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    using Core;
    using Data;
    using Util.Collections.Generic;
    
    public class IdentifierManager
    {
        #region Interfaces
        public interface IIdentifiable
        {
            #region Properties
            string ID { get; }
            #endregion
        }

        public interface IIdentifiableNamespace : IIdentifiable
        {
            #region Properties
            string Name { get; }
            IIdentifiableNamespace ParentNamespace { get; }
            List<IIdentifiableNamespace> ChildNamespaces { get; }
            #endregion

            #region Methods
            void AddChildNamespace(IIdentifiableNamespace childNamespace);
            void AddChildNamespaces(IEnumerable<IIdentifiableNamespace> childNamespaces);
            #endregion
        }

        public interface IIdentifiableType : IIdentifiable
        {
            #region Properties
            Type Type { get; }
            string TypeName { get; }
            IIdentifiableType ParentType { get; }
            List<IIdentifiableType> ChildTypes { get; }
            IIdentifiableNamespace TypeNamespace { get; }
            #endregion

            #region Methods
            void AddChildType(IIdentifiableType childType);
            void AddChildTypes(IEnumerable<IIdentifiableType> childTypes);
            #endregion
        }

        public interface IIdentifiableMetaDataType : IIdentifiableType
        {
            IIdentifiableRuntimeDataType RuntimeDataType { get; }
            IIdentifiablePersistentDataType PersistentDataType { get; }
        }
        
        public interface IIdentifiableRuntimeDataType : IIdentifiableType
        {
            
        }

        public interface IIdentifiablePersistentDataType : IIdentifiableType
        {
            
        }

        public interface IIdentifiableGameObjectType : IIdentifiableType
        {
            #region Properties
            IIdentifiableRuntimeDataType RuntimeDataType { get; }
            IIdentifiablePersistentDataType PersistentDataType { get; }
            #endregion

            #region Methods
            IPersistentData SerializeData<RuntimeDataType, PersistentDataType>(IRuntimeData runtimeData) where RuntimeDataType : IRuntimeData where PersistentDataType : IPersistentData;
            IRuntimeData DeserializeData<RuntimeDataType, PersistentDataType>(IPersistentData persistentData) where RuntimeDataType : IRuntimeData where PersistentDataType : IPersistentData;
            IIdentifiableGameObjectInstance CreateGameObjectInstance();
            #endregion
        }

        public interface IIdentifiableComponentType : IIdentifiableType
        {
            #region Properties
            IIdentifiableRuntimeDataType RuntimeDataType { get; }
            IIdentifiablePersistentDataType PersistentDataType { get; }
            #endregion

            #region Methods
            IPersistentData SerializeData<RuntimeDataType, PersistentDataType>(IRuntimeData runtimeData) where RuntimeDataType : IRuntimeData where PersistentDataType : IPersistentData;
            IRuntimeData DeserializeData<RuntimeDataType, PersistentDataType>(IPersistentData persistentData) where RuntimeDataType : IRuntimeData where PersistentDataType : IPersistentData;
            IIdentifiableComponentInstance CreateComponentInstance();
            #endregion
        }

        public interface IIdentifiableInstance : IIdentifiable
        {
            #region Properties
            IIdentifiableType InstanceType { get; }
            Guid InstanceID { get; }
            #endregion
        }

        public interface IIdentifiableGameObjectInstance : IIdentifiableInstance
        {
            #region Properties
            IIdentifiableGameObjectInstance ParentObject { get; }
            List<IIdentifiableGameObjectInstance> ChildObjects { get; }
            List<IIdentifiableComponentInstance> ChildComponents { get; }
            IRuntimeData RuntimeData { get; }
            IPersistentData PersistentData { get; }
            GameObject ObjectInstance { get; }
            #endregion

            #region Methods
            void AddChildObjectInstance(IIdentifiableGameObjectInstance childObjectInstance);
            void AddChildObjectInstances(IEnumerable<IIdentifiableGameObjectInstance> childObjectInstances);
            void RemoveChildObjectInstance(IIdentifiableGameObjectInstance childObjectInstance);
            void RemoveChildObjectInstances(IEnumerable<IIdentifiableGameObjectInstance> childObjectInstances);
            void AddChildComponentInstance(IIdentifiableComponentInstance childComponentInstance);
            void AddChildComponentInstances(IEnumerable<IIdentifiableComponentInstance> childComponentInstances);
            void RemoveChildComponentInstance(IIdentifiableComponentInstance childComponentInstance);
            void RemoveChildComponentInstances(IEnumerable<IIdentifiableComponentInstance> childComponentInstances);
            #endregion
        }

        public interface IIdentifiableComponentInstance : IIdentifiableInstance
        {
            #region Properties
            IIdentifiableGameObjectInstance ParentObject { get; }
            IRuntimeData RuntimeData { get; }
            IPersistentData PersistentData { get; }
            Component ComponentInstance { get; }
            #endregion
        }
        #endregion

        #region Classes
        [Serializable]
        public class NamespaceIdentifier : IIdentifiableNamespace
        {
            #region Properties
            public string ID
            {
                get
                {
                    return $"{FullName}";
                }
            }
            public string FullName
            {
                get
                {
                    if (ParentNamespace == null)
                    {
                        return Name;
                    }
                    else
                    {
                        return $"{ParentNamespace.Name}.{Name}";
                    }
                }
            }

            public string Name => name;
            public IIdentifiableNamespace ParentNamespace => parentNamespace;
            public List<IIdentifiableNamespace> ChildNamespaces => childNamespaces.Values;
            #endregion

            #region Fields
            [SerializeField] private string name;
            [SerializeField] private IIdentifiableNamespace parentNamespace;
            [SerializeField] private SerializableList<IIdentifiableNamespace> childNamespaces;
            #endregion

            #region Constructors
            internal NamespaceIdentifier(string name)
            {
                this.name = name;
                parentNamespace = null;
                childNamespaces = null;
            }

            internal NamespaceIdentifier(string name, IIdentifiableNamespace parentNamespace)
            {
                this.name = name;
                this.parentNamespace = parentNamespace;
                childNamespaces = null;
            }
            #endregion

            #region Methods
            public void AddChildNamespace(IIdentifiableNamespace childNamespace)
            {
                if (childNamespaces == null)
                {
                    childNamespaces = new SerializableList<IIdentifiableNamespace>();
                }

                if (childNamespaces.Contains(childNamespace))
                {
                    throw new Exception($"Namespace '{childNamespace.Name}' already exists in {Name}!");
                }

                childNamespaces.Add(childNamespace);
            }

            public void AddChildNamespaces(IEnumerable<IIdentifiableNamespace> childNamespaces)
            {
                if (this.childNamespaces == null)
                {
                    this.childNamespaces = new SerializableList<IIdentifiableNamespace>();
                }

                foreach (IIdentifiableNamespace childNamespace in childNamespaces)
                {
                    if (this.childNamespaces.Contains(childNamespace))
                    {
                        throw new Exception($"Namespace '{childNamespace.Name}' already exists in {Name}!");
                    }
                }

                this.childNamespaces.AddRange(childNamespaces);
            }
            #endregion
        }

        [Serializable]
        public class GameObjectTypeIdentifier<GameObjectType> : IIdentifiableGameObjectType where GameObjectType : GameObject
        {
            #region Properties
            public string ID
            {
                get
                {
                    return $"{TypeNamespace.ID}.{ParentType.ID}.{TypeName}";
                }
            }
            public Type Type
            {
                get
                {
                    return Type.GetType(assemblyQualifiedTypeName);
                }
            }
            public GameObjectType GameObjectInstance
            {
                get
                {
                    return gameObjectInstance;
                }
            }

            public string TypeName => typeName;
            public IIdentifiableType ParentType => parentType;
            public List<IIdentifiableType> ChildTypes => childTypes.Values;
            public IIdentifiableNamespace TypeNamespace => typeNamespace;
            public IIdentifiableRuntimeDataType RuntimeDataType => runtimeDataType;
            public IIdentifiablePersistentDataType PersistentDataType => persistentDataType;
            #endregion

            #region Fields
            [SerializeField] private string typeName;
            [SerializeField] private string assemblyQualifiedTypeName;
            [SerializeField] private IIdentifiableType parentType;
            [SerializeField] private SerializableList<IIdentifiableType> childTypes;
            [SerializeField] private IIdentifiableNamespace typeNamespace;
            [SerializeField] private IIdentifiableRuntimeDataType runtimeDataType;
            [SerializeField] private IIdentifiablePersistentDataType persistentDataType;

            private GameObjectType gameObjectInstance;
            #endregion

            #region Constructors
            internal GameObjectTypeIdentifier(IIdentifiableNamespace typeNamespace, Type type)
            {
                typeName = type.Name;
                assemblyQualifiedTypeName = type.AssemblyQualifiedName;
                parentType = null;
                childTypes = new SerializableList<IIdentifiableType>();
                this.typeNamespace = typeNamespace;
            }

            internal GameObjectTypeIdentifier(IIdentifiableType parentType, Type type)
            {
                typeName = type.Name;
                assemblyQualifiedTypeName = type.AssemblyQualifiedName;
                this.parentType = parentType;
                childTypes = new SerializableList<IIdentifiableType>();
                typeNamespace = parentType.TypeNamespace;
            }
            #endregion

            #region Methods
            public void AddChildType(IIdentifiableType childType)
            {
                if (childTypes.Contains(childType))
                {
                    throw new Exception($"[TypeIdentifier] Type '{childType.TypeName}' already exists in parent '{ID}'!");
                }
                childTypes.Add(childType);
            }

            public void AddChildTypes(IEnumerable<IIdentifiableType> childTypes)
            {
                if (this.childTypes == null)
                {
                    this.childTypes = new SerializableList<IIdentifiableType>();
                }
                foreach (IIdentifiableType childType in childTypes)
                {
                    if (this.childTypes.Contains(childType))
                    {
                        throw new ArgumentException($"[TypeIdentifier] Type '{childType.ID}' already exists in parent '{ID}'!");
                    }
                    this.childTypes.Add(childType);
                }
            }

            public IPersistentData SerializeData<RuntimeDataType, PersistentDataType>(IRuntimeData runtimeData)
                where RuntimeDataType : IRuntimeData 
                where PersistentDataType : IPersistentData
            {
                
            }
            
            public IRuntimeData DeserializeData<RuntimeDataType, PersistentDataType>(IPersistentData persistentData) 
                where RuntimeDataType : IRuntimeData 
                where PersistentDataType : IPersistentData
            {
                
            }
            
            public IIdentifiableGameObjectInstance CreateGameObjectInstance()
            {
                
            }
            #endregion
        }

        [Serializable]
        public class ComponentTypeIdentifier<ComponentType> : IIdentifiableComponentType where ComponentType : Component
        {
            #region Properties
            public string ID
            {
                get
                {
                    return $"{TypeNamespace.ID}.{ParentType.ID}.{TypeName}";
                }
            }
            public Type Type
            {
                get
                {
                    return Type.GetType(assemblyQualifiedTypeName);
                }
            }
            public ComponentType ComponentInstance
            {
                get
                {
                    return componentInstance;
                }
            }

            public string TypeName => typeName;
            public IIdentifiableType ParentType => parentType;
            public List<IIdentifiableType> ChildTypes => childTypes.Values;
            public IIdentifiableNamespace TypeNamespace => typeNamespace;
            public IIdentifiableMetaDataType MetaDataType => metaDataType;
            public IIdentifiableRuntimeDataType RuntimeDataType => runtimeDataType;
            public IIdentifiablePersistentDataType PersistentDataType => persistentDataType;
            #endregion

            #region Fields
            [SerializeField] private string typeName;
            [SerializeField] private string assemblyQualifiedTypeName;
            [SerializeField] private IIdentifiableType parentType;
            [SerializeField] private SerializableList<IIdentifiableType> childTypes;
            [SerializeField] private IIdentifiableNamespace typeNamespace;
            [SerializeField] private IIdentifiableMetaDataType metaDataType;
            [SerializeField] private IIdentifiableRuntimeDataType runtimeDataType;
            [SerializeField] private IIdentifiablePersistentDataType persistentDataType;

            private ComponentType componentInstance;
            #endregion

            #region Constructors
            internal ComponentTypeIdentifier(IIdentifiableNamespace typeNamespace, Type type, IIdentifiableRuntimeDataType runtimeDataType, IIdentifiablePersistentDataType persistentDataType)
            {
                typeName = type.Name;
                assemblyQualifiedTypeName = type.AssemblyQualifiedName;
                parentType = null;
                childTypes = new SerializableList<IIdentifiableType>();
                this.typeNamespace = typeNamespace;
            }

            internal ComponentTypeIdentifier(IIdentifiableType parentType, Type type)
            {
                typeName = type.Name;
                assemblyQualifiedTypeName = type.AssemblyQualifiedName;
                this.parentType = parentType;
                childTypes = new SerializableList<IIdentifiableType>();
                typeNamespace = parentType.TypeNamespace;
            }
            #endregion

            #region Methods
            public void AddChildType(IIdentifiableType childType)
            {
                if (childTypes.Contains(childType))
                {
                    throw new Exception($"[TypeIdentifier] Type '{childType.TypeName}' already exists in parent '{ID}'!");
                }
                childTypes.Add(childType);
            }

            public void AddChildTypes(IEnumerable<IIdentifiableType> childTypes)
            {
                if (this.childTypes == null)
                {
                    this.childTypes = new SerializableList<IIdentifiableType>();
                }
                foreach (IIdentifiableType childType in childTypes)
                {
                    if (this.childTypes.Contains(childType))
                    {
                        throw new ArgumentException($"[TypeIdentifier] Type '{childType.ID}' already exists in parent '{ID}'!");
                    }
                    this.childTypes.Add(childType);
                }
            }

            public IPersistentData SerializeData<RuntimeDataType, PersistentDataType>(IRuntimeData runtimeData)
                where RuntimeDataType : IRuntimeData
                where PersistentDataType : IPersistentData
            {
                
            }

            public IRuntimeData DeserializeData<RuntimeDataType, PersistentDataType>(IPersistentData persistentData)
                where RuntimeDataType : IRuntimeData
                where PersistentDataType : IPersistentData
            {
                
            }

            public IIdentifiableComponentInstance CreateComponentInstance()
            {

            }
            #endregion
        }

        [Serializable]
        public class InstanceIdentifier : IIdentifiableInstance
        {
            #region Properties
            public string ID
            {
                get
                {
                    if (ParentInstance == null)
                    {
                        return $"{InstanceType.ID}.{InstanceID}";
                    }
                    else
                    {
                        return $"{ParentInstance.ID}.{InstanceType.ID}.{InstanceID}";
                    }
                }
            }

            public IIdentifiableInstance ParentInstance => parentInstance;
            public List<IIdentifiableInstance> ChildInstances => childInstances.Values;
            public IIdentifiableType InstanceType => instanceType;
            public Guid InstanceID => instanceID;
            #endregion

            #region Fields
            [SerializeField] private IIdentifiableInstance parentInstance;
            [SerializeField] private SerializableList<IIdentifiableInstance> childInstances;
            [SerializeField] private IIdentifiableType instanceType;
            [SerializeField] private Guid instanceID;
            #endregion

            #region Constructors
            internal InstanceIdentifier(IIdentifiableType instanceType, Guid instanceID)
            {
                this.instanceType = instanceType;
                this.instanceID = instanceID;
                parentInstance = null;
                childInstances = null;
            }

            internal InstanceIdentifier(IIdentifiableType instanceType, Guid instanceID, IIdentifiableInstance parentInstance)
            {
                this.instanceType = instanceType;
                this.instanceID = instanceID;
                this.parentInstance = parentInstance;
                childInstances = null;
            }
            #endregion

            #region Methods
            public void AddChildInstance(IIdentifiableInstance childInstance)
            {
                if (childInstances == null)
                {
                    childInstances = new SerializableList<IIdentifiableInstance>();
                }
                if (childInstances.Contains(childInstance))
                {
                    throw new ArgumentException($"Child instance '{childInstance.ID}' already exists in '{ID}'!");
                }
                childInstances.Add(childInstance);
            }

            public void AddChildInstances(IEnumerable<IIdentifiableInstance> childInstances)
            {
                if (this.childInstances == null)
                {
                    this.childInstances = new SerializableList<IIdentifiableInstance>();
                }
                foreach (IIdentifiableInstance childInstance in childInstances)
                {
                    if (this.childInstances.Contains(childInstance))
                    {
                        throw new ArgumentException($"Child instance '{childInstance.ID}' already exists in '{ID}'!");
                    }
                    this.childInstances.Add(childInstance);
                }
            }

            public void RemoveChildInstance(IIdentifiableInstance childInstance)
            {
                if (childInstances == null)
                {
                    throw new NullReferenceException();
                }
                if (!childInstances.Contains(childInstance))
                {
                    throw new ArgumentException($"Child instance '{childInstance.ID}' does not exist in '{ID}'!");
                }
                childInstances.Remove(childInstance);
            }
            #endregion
        }
        #endregion

        #region Static Properties
        public static IdentifierManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new IdentifierManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static IdentifierManager instance;
        #endregion

        #region Properties
        public Dictionary<string, IIdentifiableNamespace> RootNamespaces => rootNamespaces;
        public Dictionary<string, IIdentifiableType> RootTypes => rootTypes;
        public Dictionary<string, IIdentifiableInstance> RootInstances => rootInstances;
        #endregion

        #region Fields
        private Dictionary<string, IIdentifiableNamespace> rootNamespaces;
        private Dictionary<string, IIdentifiableType> rootTypes;
        private Dictionary<string, IIdentifiableInstance> rootInstances;
        #endregion

        internal void Initialize()
        {
            rootNamespaces = new Dictionary<string, IIdentifiableNamespace>();
            rootTypes = new Dictionary<string, IIdentifiableType>();
            rootInstances = new Dictionary<string, IIdentifiableInstance>();
        }

        public IIdentifiableNamespace CreateRootNamespace(string namespaceName)
        {
            if (string.IsNullOrEmpty(namespaceName))
            {
                throw new Exception($"[IdentifierManager] Namespace name cannot be empty!");
            }
            if (rootNamespaces.ContainsKey(namespaceName))
            {
                throw new Exception($"[IdentifierManager] Root Namespace '{namespaceName}' already exists!");
            }
            NamespaceIdentifier newRootNamespace = new NamespaceIdentifier(namespaceName);
            rootNamespaces.Add(newRootNamespace.ID, newRootNamespace);
            return newRootNamespace;
        }
        
        public IIdentifiableNamespace CreateNamespace(IIdentifiableNamespace parentNamespace, string namespaceName)
        {
            NamespaceIdentifier newChildNamespace = new NamespaceIdentifier(namespaceName, parentNamespace);
            if (string.IsNullOrEmpty(namespaceName))
            {
                throw new Exception($"[IdentifierManager] Namespace name cannot be empty!");
            }
            if (parentNamespace.ChildNamespaces.Contains(newChildNamespace))
            {
                throw new Exception($"[IdentifierManager] Namespace '{namespaceName}' in parent '{parentNamespace.ID}' already exists!");
            }
            parentNamespace.AddChildNamespace(newChildNamespace);
            return newChildNamespace;
        }

        public IIdentifiableType CreateRootGameObjectType(IIdentifiableNamespace parentNamespace, Type type)
        {
            if (rootTypes.ContainsKey(type.Name))
            {
                throw new Exception($"[IdentifierManager] Root Type '{type.Name}' already exists!");
            }
            GameObjectTypeIdentifier newRootType = new GameObjectTypeIdentifier(parentNamespace, type);
            rootTypes.Add(newRootType.ID, newRootType);
            return newRootType;
        }

        public IIdentifiableType CreateGameObjectType(IIdentifiableGameObjectType parentType, Type type)
        {
            GameObjectTypeIdentifier newChildType = new GameObjectTypeIdentifier(parentType, type);
            if (parentType.ChildTypes.Contains(newChildType))
            {
                throw new Exception($"[IdentifierManager] Type '{type.Name}' in parent '{parentType.ID}' already exists!");
            }
            parentType.AddChildType(newChildType);
            return newChildType;
        }

        public IIdentifiableType CreateRootComponentType(IIdentifiableNamespace parentNamespace, Type type)
        {
            if (rootTypes.ContainsKey(type.Name))
            {
                throw new Exception($"[IdentifierManager] Root Type '{type.Name}' already exists!");
            }
            ComponentTypeIdentifier newRootType = new ComponentTypeIdentifier(parentNamespace, type);
            rootTypes.Add(newRootType.ID, newRootType);
            return newRootType;
        }

        public IIdentifiableType CreateComponentType(IIdentifiableComponentType parentType, Type type)
        {
            ComponentTypeIdentifier newChildType = new ComponentTypeIdentifier(parentType, type);
            if (parentType.ChildTypes.Contains(newChildType))
            {
                throw new Exception($"[IdentifierManager] Type '{type.Name}' in parent '{parentType.ID}' already exists!");
            }
            parentType.AddChildType(newChildType);
            return newChildType;
        }

        public IIdentifiableInstance AddRootInstance(IIdentifiableType parentType, Component objectInstance)
        {
            InstanceIdentifier newRootInstance = new InstanceIdentifier(parentType, objectInstance.InstanceID);
            if (rootInstances.ContainsKey(newRootInstance.ID))
            {
                throw new Exception($"[IdentifierManager] Root Instance '{newRootInstance.ID}' already exists!");
            }
            rootInstances.Add(newRootInstance.ID, newRootInstance);
            return newRootInstance;
        }

        public IIdentifiableInstance AddInstance(IIdentifiableType parentType, Component instance, IIdentifiableInstance parentInstance)
        {
            InstanceIdentifier newChildInstance = new InstanceIdentifier(parentType, instance.InstanceID, parentInstance);
            if (parentInstance.ChildInstances.Contains(newChildInstance))
            {
                throw new Exception($"[IdentifierManager] Instance '{newChildInstance.ID}' in parent '{parentInstance.ID}' already exists!");
            }
            parentInstance.AddChildInstance(newChildInstance);
            return newChildInstance;
        }
    } 
}