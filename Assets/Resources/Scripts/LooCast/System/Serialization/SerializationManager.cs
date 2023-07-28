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
        private Dictionary<Type, Serializer> serializers;
        #endregion

        #region Constructors
        public SerializationManager() : base()
        {
            serializers = new Dictionary<Type, Serializer>();
            
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
                RegisterSerializer();

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
        public void RegisterSerializer(Serializer serializer)
        {
            if (serializer == null)
            {
                throw new ArgumentNullException("serializer");
            }

            if (serializers.ContainsKey(serializer.SerializableType))
            {
                throw new ArgumentException($"Serializer already registered for type '{serializer.SerializableType}'!");
            }

            serializers.Add(serializer.SerializableType, serializer);
        }

        public Serializer GetSerializer(Type type)
        {
            if (type == null)
            {
                throw new ArgumentNullException("type");
            }

            if (!serializers.ContainsKey(type))
            {
                throw new ArgumentException($"No serializer registered for type '{type}'!");
            }

            return serializers[type];
        }
        #endregion
    }
}
