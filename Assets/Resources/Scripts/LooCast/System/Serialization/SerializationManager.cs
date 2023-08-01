using System;
using System.Reflection;
using System.Linq;
using System.Collections.Generic;

namespace LooCast.System.Serialization
{
    using LooCast.System.ECS;

    public sealed class SerializationManager : ModuleManager
    {
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

                foreach (Assembly assembly in AppDomain.CurrentDomain.GetAssemblies())
                {
                    foreach (Type type in assembly.GetTypes())
                    {
                        CacheSerializability(type);
                    }
                }

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
        public bool IsPrimitiveAttributeType(Type type)
        {
            return primitiveAttributeSerializers.ContainsKey(type);
        }
        
        public bool IsPrimitiveObjectType(Type type)
        {
            return primitiveObjectSerializers.ContainsKey(type);
        }

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

        public void CacheSerializability(Type type)
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
            }
            else
            {
                serializabilityCache.Add(type, Serializability.None);
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
        #endregion
    }
}
