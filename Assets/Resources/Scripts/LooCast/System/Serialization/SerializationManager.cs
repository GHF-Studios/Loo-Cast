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
        private Dictionary<Type, IPrimitiveSerializer> primitiveSerializers;
        private Dictionary<Type, ICompositeFolderSerializer> compositeFolderSerializers;
        private Dictionary<Type, ICompositeFileSerializer> compositeFileSerializers;
        private Dictionary<Type, ICompositeObjectSerializer> compositeObjectSerializers;
        #endregion

        #region Constructors
        public SerializationManager() : base()
        {
            primitiveSerializers = new Dictionary<Type, IPrimitiveSerializer>();
            compositeFolderSerializers = new Dictionary<Type, ICompositeFolderSerializer>();
            compositeFileSerializers = new Dictionary<Type, ICompositeFileSerializer>();
            compositeObjectSerializers = new Dictionary<Type, ICompositeObjectSerializer>();

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

                #region Serializer Registration
                RegisterPrimitiveSerializer(new BoolPrimitiveSerializer());
                RegisterPrimitiveSerializer(new BytePrimitiveSerializer());
                RegisterPrimitiveSerializer(new SBytePrimitiveSerializer());
                RegisterPrimitiveSerializer(new CharPrimitiveSerializer());
                RegisterPrimitiveSerializer(new DecimalPrimitiveSerializer());
                RegisterPrimitiveSerializer(new DoublePrimitiveSerializer());
                RegisterPrimitiveSerializer(new FloatPrimitiveSerializer());
                RegisterPrimitiveSerializer(new IntPrimitiveSerializer());
                RegisterPrimitiveSerializer(new UIntPrimitiveSerializer());
                RegisterPrimitiveSerializer(new LongPrimitiveSerializer());
                RegisterPrimitiveSerializer(new ULongPrimitiveSerializer());
                RegisterPrimitiveSerializer(new ShortPrimitiveSerializer());
                RegisterPrimitiveSerializer(new UShortPrimitiveSerializer());
                RegisterPrimitiveSerializer(new StringPrimitiveSerializer());
                RegisterPrimitiveSerializer(new BigIntPrimitiveSerializer());

                Assembly[] assemblies = AppDomain.CurrentDomain.GetAssemblies();
                IEnumerable<Type> types = assemblies.SelectMany(assembly => assembly.GetTypes());
                IEnumerable<Type> entityTypes = types.Where(type => typeof(IEntity).IsAssignableFrom(type));
                IEnumerable<Type> componentTypes = types.Where(type => typeof(IComponent).IsAssignableFrom(type));

                // automatically assign each composite type a composite serializer and also somehow determine if a type should be serialized in binary
                // maybe add some attributes for this, once the system is kind of up and running
                // the representation of the attributes and the relations to each other is literally the sets of serializers
                #endregion

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
        public bool IsPrimitiveType(Type type)
        {
            if (primitiveSerializers.ContainsKey(type))
            {
                return true;
            }
            else if (compositeFolderSerializers.ContainsKey(type))
            {
                return false;
            }
            else if (compositeFileSerializers.ContainsKey(type))
            {
                return false;
            }
            else if (compositeObjectSerializers.ContainsKey(type))
            {
                return false;
            }
            else
            {
                throw new Exception($"Type '{type}' is not a registered serializable type!");
            }
        }

        public bool IsCompositeFolderType(Type type)
        {
            if (compositeFolderSerializers.ContainsKey(type))
            {
                return true;
            }
            else if (compositeFileSerializers.ContainsKey(type))
            {
                return false;
            }
            else if (compositeObjectSerializers.ContainsKey(type))
            {
                return false;
            }
            else if (primitiveSerializers.ContainsKey(type))
            {
                return false;
            }
            else
            {
                throw new Exception($"Type '{type}' is not a registered serializable type!");
            }
        }

        public bool IsCompositeFileType(Type type)
        {
            if (compositeFileSerializers.ContainsKey(type))
            {
                return true;
            }
            else if (compositeObjectSerializers.ContainsKey(type))
            {
                return false;
            }
            else if (primitiveSerializers.ContainsKey(type))
            {
                return false;
            }
            else if (compositeFolderSerializers.ContainsKey(type))
            {
                return false;
            }
            else
            {
                throw new Exception($"Type '{type}' is not a registered serializable type!");
            }
        }

        public bool IsCompositeObjectType(Type type)
        {
            if (compositeObjectSerializers.ContainsKey(type))
            {
                return true;
            }
            else if (primitiveSerializers.ContainsKey(type))
            {
                return false;
            }
            else if (compositeFolderSerializers.ContainsKey(type))
            {
                return false;
            }
            else if (compositeFileSerializers.ContainsKey(type))
            {
                return false;
            }
            else
            {
                throw new Exception($"Type '{type}' is not a registered serializable type!");
            }
        }
        
        public void RegisterPrimitiveSerializer(IPrimitiveSerializer primitiveSerializer)
        {
            if (primitiveSerializer == null)
            {
                throw new ArgumentNullException(nameof(primitiveSerializer));
            }

            if (primitiveSerializers.ContainsKey(primitiveSerializer.SerializableType))
            {
                throw new ArgumentException($"Primitive serializer already registered for type '{primitiveSerializer.SerializableType}'!");
            }

            primitiveSerializers.Add(primitiveSerializer.SerializableType, primitiveSerializer);
        }

        public void RegisterCompositeFolderSerializer(ICompositeFolderSerializer compositeFolderSerializer)
        {
            if (compositeFolderSerializer == null)
            {
                throw new ArgumentNullException(nameof(compositeFolderSerializer));
            }

            if (compositeFolderSerializers.ContainsKey(compositeFolderSerializer.SerializableType))
            {
                throw new ArgumentException($"Composite folder serializer already registered for type '{compositeFolderSerializer.SerializableType}'!");
            }

            compositeFolderSerializers.Add(compositeFolderSerializer.SerializableType, compositeFolderSerializer);
        }

        public void RegisterCompositeFileSerializer(ICompositeFileSerializer compositeFileSerializer)
        {
            if (compositeFileSerializer == null)
            {
                throw new ArgumentNullException(nameof(compositeFileSerializer));
            }

            if (compositeFileSerializers.ContainsKey(compositeFileSerializer.SerializableType))
            {
                throw new ArgumentException($"Composite file serializer already registered for type '{compositeFileSerializer.SerializableType}'!");
            }

            compositeFileSerializers.Add(compositeFileSerializer.SerializableType, compositeFileSerializer);
        }

        public void RegisterCompositeObjectSerializer(ICompositeObjectSerializer compositeObjectSerializer)
        {
            if (compositeObjectSerializer == null)
            {
                throw new ArgumentNullException(nameof(compositeObjectSerializer));
            }

            if (compositeObjectSerializers.ContainsKey(compositeObjectSerializer.SerializableType))
            {
                throw new ArgumentException($"Composite object serializer already registered for type '{compositeObjectSerializer.SerializableType}'!");
            }

            compositeObjectSerializers.Add(compositeObjectSerializer.SerializableType, compositeObjectSerializer);
        }

        public IPrimitiveSerializer GetPrimitiveSerializer(Type type)
        {
            if (type == null)
            {
                throw new ArgumentNullException("type");
            }

            if (!primitiveSerializers.ContainsKey(type))
            {
                throw new ArgumentException($"No primitive serializer registered for type '{type}'!");
            }

            return primitiveSerializers[type];
        }

        public ICompositeFolderSerializer GetCompositeFolderSerializer(Type type)
        {
            if (type == null)
            {
                throw new ArgumentNullException("type");
            }

            if (!compositeFolderSerializers.ContainsKey(type))
            {
                throw new ArgumentException($"No composite folder serializer registered for type '{type}'!");
            }

            return compositeFolderSerializers[type];
        }

        public ICompositeFileSerializer GetCompositeFileSerializer(Type type)
        {
            if (type == null)
            {
                throw new ArgumentNullException("type");
            }

            if (!compositeFileSerializers.ContainsKey(type))
            {
                throw new ArgumentException($"No composite file serializer registered for type '{type}'!");
            }

            return compositeFileSerializers[type];
        }

        public ICompositeObjectSerializer GetCompositeObjectSerializer(Type type)
        {
            if (type == null)
            {
                throw new ArgumentNullException("type");
            }

            if (!compositeObjectSerializers.ContainsKey(type))
            {
                throw new ArgumentException($"No composite object serializer registered for type '{type}'!");
            }

            return compositeObjectSerializers[type];
        }
        #endregion
    }
}
