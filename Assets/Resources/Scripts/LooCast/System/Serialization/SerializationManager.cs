using System;
using System.Reflection;
using System.Linq;
using System.Collections.Generic;

namespace LooCast.System.Serialization
{
    using LooCast.System.ECS;
    using LooCast.System.Paths;

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
        private Dictionary<Type, ICompositeSerializer> compositeSerializers;
        #endregion

        #region Constructors
        public SerializationManager() : base()
        {
            primitiveSerializers = new Dictionary<Type, IPrimitiveSerializer>();
            compositeSerializers = new Dictionary<Type, ICompositeSerializer>();
            
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
            else if (compositeSerializers.ContainsKey(type))
            {
                return false;
            }
            else
            {
                throw new Exception($"Type '{type}' is not registered in the SerializationManager!");
            }
        }

        public bool IsCompositeType(Type type)
        {
            if (compositeSerializers.ContainsKey(type))
            {
                return true;
            }
            else if (primitiveSerializers.ContainsKey(type))
            {
                return false;
            }
            else
            {
                throw new Exception($"Type '{type}' is not registered in the SerializationManager!");
            }
        }
        
        public void RegisterPrimitiveSerializer(IPrimitiveSerializer primitiveSerializer)
        {
            if (primitiveSerializer == null)
            {
                throw new ArgumentNullException("serializer");
            }

            if (primitiveSerializers.ContainsKey(primitiveSerializer.SerializableType))
            {
                throw new ArgumentException($"Primitive serializer already registered for type '{primitiveSerializer.SerializableType}'!");
            }

            primitiveSerializers.Add(primitiveSerializer.SerializableType, primitiveSerializer);
        }

        public void RegisterCompositeSerializer(ICompositeSerializer compositeSerializer)
        {
            if (compositeSerializer == null)
            {
                throw new ArgumentNullException("serializer");
            }

            if (compositeSerializers.ContainsKey(compositeSerializer.SerializableType))
            {
                throw new ArgumentException($"Composite serializer already registered for type '{compositeSerializer.SerializableType}'!");
            }

            compositeSerializers.Add(compositeSerializer.SerializableType, compositeSerializer);
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

        public ICompositeSerializer GetCompositeSerializer(Type type)
        {
            if (type == null)
            {
                throw new ArgumentNullException("type");
            }

            if (!compositeSerializers.ContainsKey(type))
            {
                throw new ArgumentException($"No composite serializer registered for type '{type}'!");
            }

            return compositeSerializers[type];
        }
        #endregion
    }
}
