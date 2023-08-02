using System;
using System.Reflection;
using System.Linq;
using System.Collections.Generic;
using System.Diagnostics;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    using global::System.IO;
    using LooCast.System.ECS;

    public sealed class SerializationManager : ModuleManager
    {
        #region Delegates
        public delegate void SerializePrimitiveAttributeDelegate(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute);
        public delegate void DeserializePrimitiveAttributeDelegate(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute);

        public delegate void SerializePrimitiveObjectDelegate(string primitiveObjectName, object primitiveObject, out XElement serializedPrimitiveObject);
        public delegate void DeserializePrimitiveObjectDelegate(XElement serializedPrimitiveObject, out object primitiveObject);

        public delegate void SerializeAttributeDelegate(string attributeName, object attribute, out XAttribute serializedAttribute);
        public delegate void DeserializeAttributeDelegate(XAttribute serializedAttribute, out object attribute);

        public delegate void SerializeObjectDelegate(string objectName, object _object, out XElement serializedObject);
        public delegate void DeserializeObjectDelegate(XElement serializedObject, out object _object);

        public delegate void SerializeFileDelegate(string fileName, string fileExtension, object file, out FileInfo serializedFile);
        public delegate void DeserializeFileDelegate(FileInfo serializedFile, out object file);

        public delegate void SerializeFolderDelegate(string folderName, object folder, out DirectoryInfo serializedFolder);
        public delegate void DeserializeFolderDelegate(DirectoryInfo serializedFolder, out object folder);
        #endregion
        
        #region Static Properties
        public static SerializationManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<SerializationManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static SerializationManager instance;
        #endregion

        #region Fields
        private Dictionary<Type, IPrimitiveAttributeSerializer> primitiveAttributeSerializers;
        private Dictionary<Type, IPrimitiveObjectSerializer> primitiveObjectSerializers;
        private Dictionary<Type, Serializability> serializabilityCache;

        private Dictionary<Type, SerializePrimitiveAttributeDelegate> primitiveAttributeSerializationDelegates;
        private Dictionary<Type, DeserializePrimitiveAttributeDelegate> primitiveAttributeDeserializationDelegates;

        private Dictionary<Type, SerializePrimitiveObjectDelegate> primitiveObjectSerializationDelegates;
        private Dictionary<Type, DeserializePrimitiveObjectDelegate> primitiveObjectDeserializationDelegates;

        private Dictionary<Type, SerializeAttributeDelegate> attributeSerializationDelegates;
        private Dictionary<Type, DeserializeAttributeDelegate> attributeDeserializationDelegates;

        private Dictionary<Type, SerializeObjectDelegate> objectSerializationDelegates;
        private Dictionary<Type, DeserializeObjectDelegate> objectDeserializationDelegates;

        private Dictionary<Type, SerializeFileDelegate> fileSerializationDelegates;
        private Dictionary<Type, DeserializeFileDelegate> fileDeserializationDelegates;

        private Dictionary<Type, SerializeFolderDelegate> folderSerializationDelegates;
        private Dictionary<Type, DeserializeFolderDelegate> folderDeserializationDelegates;
        #endregion

        #region Constructors
        public SerializationManager() : base()
        {
            primitiveAttributeSerializers = new Dictionary<Type, IPrimitiveAttributeSerializer>();
            primitiveObjectSerializers = new Dictionary<Type, IPrimitiveObjectSerializer>();
            serializabilityCache = new Dictionary<Type, Serializability>();

            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedComponentManagerEntityTypeName = typeof(SerializationManager).AssemblyQualifiedName;
                string assemblyQualifiedComponentManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedComponentManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData componentManagerMetaData = new Entity.MetaData();
                componentManagerMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedComponentManagerEntityTypeName;
                componentManagerMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedComponentManagerEntityMetaDataTypeName;
                componentManagerMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedComponentManagerEntityDataTypeName;
                componentManagerMetaData.EntityID = new Guid();

                Manager.Data componentManagerData = new Manager.Data();
                componentManagerData.AssemblyQualifiedEntityTypeName = assemblyQualifiedComponentManagerEntityTypeName;
                componentManagerData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedComponentManagerEntityMetaDataTypeName;
                componentManagerData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedComponentManagerEntityDataTypeName;
                componentManagerData.ManagerName = "SerializationManager";
                componentManagerData.ManagerParent = SystemManager.Instance;

                SetEntityMetaData(componentManagerMetaData);
                SetEntityData(componentManagerData);

                RegisterPrimitiveAttributeSerializer(BoolSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(ByteSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(SByteSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(CharSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(DecimalSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(DoubleSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(FloatSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(IntSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(UIntSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(LongSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(ULongSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(ShortSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(UShortSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(StringSerializer.Instance);
                RegisterPrimitiveAttributeSerializer(BigIntSerializer.Instance);

                Stopwatch stopwatch = new Stopwatch();
                stopwatch.Start();
                IEnumerable<Assembly> allAssemblies = AppDomain.CurrentDomain.GetAssemblies();
                IEnumerable<Type> allTypes = allAssemblies.SelectMany(assembly => assembly.GetTypes());
                foreach (Type type in allTypes)
                {
                    CacheSerializationInformation(type);
                }
                stopwatch.Stop();
                UnityEngine.Debug.Log($"Caching {allTypes.Count()} type's serialization information for {allAssemblies.Count()} assemblies took {stopwatch.ElapsedMilliseconds}ms");

                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                // Set pre-included components' metaData here

                // Set pre-included component'assembly data here

                // Register pre-included components here

                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPostSetup();
                }
            });

            RegisterPreInitializationAction(() =>
            {
                // Pre-Initialize pre-included components here
            });

            RegisterInitializationAction(() =>
            {
                // Initialize pre-included components here
            });

            RegisterPostInitializationAction(() =>
            {
                // Post-Initialize pre-included components here
            });
        }
        #endregion

        #region Methods
        public Serializability GetSerializability(Type type)
        {
            return serializabilityCache[type];
        }

        public void RegisterPrimitiveAttributeSerializer(IPrimitiveAttributeSerializer primitiveAttributeSerializer)
        {
            if (primitiveAttributeSerializer == null)
            {
                throw new ArgumentNullException(nameof(primitiveAttributeSerializer));
            }

            if (primitiveAttributeSerializers.ContainsKey(primitiveAttributeSerializer.PrimitiveAttributeType))
            {
                throw new ArgumentException($"Primitive attribute serializer already registered for type '{primitiveAttributeSerializer.PrimitiveAttributeType}'!");
            }

            primitiveAttributeSerializers.Add(primitiveAttributeSerializer.PrimitiveAttributeType, primitiveAttributeSerializer);
        }
        public void RegisterPrimitiveObjectSerializer(IPrimitiveObjectSerializer primitiveObjectSerializer)
        {
            if (primitiveObjectSerializer == null)
            {
                throw new ArgumentNullException(nameof(primitiveObjectSerializer));
            }

            if (primitiveObjectSerializers.ContainsKey(primitiveObjectSerializer.PrimitiveObjectType))
            {
                throw new ArgumentException($"Primitive object serializer already registered for type '{primitiveObjectSerializer.PrimitiveObjectType}'!");
            }

            primitiveObjectSerializers.Add(primitiveObjectSerializer.PrimitiveObjectType, primitiveObjectSerializer);
        }

        public IPrimitiveAttributeSerializer GetPrimitiveAttributeSerializer(Type primitiveAttributeType)
        {
            if (primitiveAttributeType == null)
            {
                throw new ArgumentNullException("type");
            }

            if (!primitiveAttributeSerializers.ContainsKey(primitiveAttributeType))
            {
                throw new ArgumentException($"No primitive attribute serializer registered for type '{primitiveAttributeType}'!");
            }

            return primitiveAttributeSerializers[primitiveAttributeType];
        }
        public IPrimitiveObjectSerializer GetPrimitiveObjectSerializer(Type primitiveObjectType)
        {
            if (primitiveObjectType == null)
            {
                throw new ArgumentNullException("type");
            }

            if (!primitiveObjectSerializers.ContainsKey(primitiveObjectType))
            {
                throw new ArgumentException($"No primitive object serializer registered for type '{primitiveObjectType}'!");
            }

            return primitiveObjectSerializers[primitiveObjectType];
        }

        public void CacheSerializationInformation(Type type)
        {
            if (type == null)
            {
                throw new ArgumentNullException(nameof(type));
            }

            if (serializabilityCache.ContainsKey(type))
            {
                throw new InvalidOperationException($"Serializability already cached for type '{type}'!");
            }

            if (type.IsPublic && !type.IsAbstract && (type.IsClass || type.IsValueType || type.IsEnum) && type.GetConstructor(Type.EmptyTypes) != null)
            {
                Serializability serializability = AnalyzeSerializability(type);
                serializabilityCache.Add(type, serializability);
                if (serializability != Serializability.None)
                {
                    CacheSerializationDelegate(type);
                }
            }
            else
            {
                serializabilityCache.Add(type, Serializability.None);
            }
        }

        private void CacheSerializationDelegate(Type serializableType)
        {
            Serializability serializability = serializabilityCache[serializableType];
            switch (serializability)
            {
                case Serializability.PrimitiveAttribute:
                    SerializePrimitiveAttributeDelegate serializePrimitiveAttributeDelegate = GeneratePrimitiveAttributeSerializationDelegate(serializableType);
                    DeserializePrimitiveAttributeDelegate deserializePrimitiveAttributeDelegate = GeneratePrimitiveAttributeDeserializationDelegate(serializableType);
                    primitiveAttributeSerializationDelegates.Add(serializableType, serializePrimitiveAttributeDelegate);
                    primitiveAttributeDeserializationDelegates.Add(serializableType, deserializePrimitiveAttributeDelegate);
                    break;
                case Serializability.PrimitiveObject:
                    SerializePrimitiveObjectDelegate serializePrimitiveObjectDelegate = GeneratePrimitiveObjectSerializationDelegate(serializableType);
                    DeserializePrimitiveObjectDelegate deserializePrimitiveObjectDelegate = GeneratePrimitiveObjectDeserializationDelegate(serializableType);
                    primitiveObjectSerializationDelegates.Add(serializableType, serializePrimitiveObjectDelegate);
                    primitiveObjectDeserializationDelegates.Add(serializableType, deserializePrimitiveObjectDelegate);
                    break;
                case Serializability.Attribute:
                    // if the serializableType contains any serializable property or field types, which are not primitive, throw an exception because attributes are indivisible building blocks
                    SerializeAttributeDelegate serializeAttributeDelegate = GenerateAttributeSerializationDelegate(serializableType);
                    DeserializeAttributeDelegate deserializeAttributeDelegate = GenerateAttributeDeserializationDelegate(serializableType);
                    attributeSerializationDelegates.Add(serializableType, serializeAttributeDelegate);
                    attributeDeserializationDelegates.Add(serializableType, deserializeAttributeDelegate);
                    break;
                case Serializability.Object:
                    // if the serializableType contains any serializable property or field object types, cache the object serialization delegates for each of them
                    break;
                case Serializability.File:
                    break;
                case Serializability.Folder:
                    break;
            }
        }

        private Serializability AnalyzeSerializability(Type type)
        {
            List<Serializability> detectedSerializabilities = new List<Serializability>();

            if (IsPrimitiveAttributeType(type))
            {
                detectedSerializabilities.Add(Serializability.PrimitiveAttribute);
            }
            if (IsPrimitiveObjectType(type))
            {
                detectedSerializabilities.Add(Serializability.PrimitiveObject);
            }

            if (typeof(ISerializableFolder).IsAssignableFrom(type))
            {
                detectedSerializabilities.Add(Serializability.Folder);
            }
            if (typeof(ISerializableFile).IsAssignableFrom(type))
            {
                detectedSerializabilities.Add(Serializability.File);
            }
            if (typeof(ISerializableObject).IsAssignableFrom(type))
            {
                detectedSerializabilities.Add(Serializability.Object);
            }
            if (typeof(ISerializableAttribute).IsAssignableFrom(type))
            {
                detectedSerializabilities.Add(Serializability.Attribute);
            }

            if (detectedSerializabilities.Count > 1)
            {
                throw new InvalidOperationException($"Type {type.FullName} has conflicting serializability!");
            }
            else if (detectedSerializabilities.Count == 1)
            {
                return detectedSerializabilities[0];
            }

            return Serializability.None;
        }

        #region (De-)serialization delegates generation
        private SerializePrimitiveAttributeDelegate GeneratePrimitiveAttributeSerializationDelegate(Type serializableType)
        {
            
        }
        private DeserializePrimitiveAttributeDelegate GeneratePrimitiveAttributeDeserializationDelegate(Type serializableType)
        {
            
        }

        private SerializePrimitiveObjectDelegate GeneratePrimitiveObjectSerializationDelegate(Type serializableType)
        {

        }
        private DeserializePrimitiveObjectDelegate GeneratePrimitiveObjectDeserializationDelegate(Type serializableType)
        {

        }

        private SerializeAttributeDelegate GenerateAttributeSerializationDelegate(Type serializableType)
        {

        }
        private DeserializeAttributeDelegate GenerateAttributeDeserializationDelegate(Type serializableType)
        {

        }

        private SerializeObjectDelegate GenerateObjectSerializationDelegate(Type serializableType)
        {

        }
        private DeserializeObjectDelegate GenerateObjectDeserializationDelegate(Type serializableType)
        {

        }

        private SerializeFileDelegate GenerateFileSerializationDelegate(Type serializableType)
        {

        }
        private DeserializeFileDelegate GenerateFileDeserializationDelegate(Type serializableType)
        {

        }

        private SerializeFolderDelegate GenerateFolderSerializationDelegate(Type serializableType)
        {

        }
        private DeserializeFolderDelegate GenerateFolderDeserializationDelegate(Type serializableType)
        {

        }
        #endregion

        #endregion
    }
}
