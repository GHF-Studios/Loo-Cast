using System;
using System.IO;
using System.Reflection;
using System.Linq;
using System.Collections.Generic;
using System.Diagnostics;
using System.Xml.Linq;
using System.Numerics;

namespace LooCast.System.Serialization
{
    using LooCast.System.ECS;
    using UnityEditor.ShaderGraph.Internal;

    public sealed class SerializationManager : ModuleManager
    {
        #region Delegates
        public delegate void SerializePrimitiveDelegate(string primitiveName, object primitive, out XAttribute serializedPrimitive);
        public delegate void DeserializePrimitiveDelegate(XAttribute serializedPrimitive, out object primitive);

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
        private Dictionary<Type, Serializability> serializabilityCache;

        private Dictionary<Type, SerializePrimitiveDelegate> primitiveSerializationDelegates;
        private Dictionary<Type, DeserializePrimitiveDelegate> primitiveDeserializationDelegates;

        private Dictionary<Type, SerializeObjectDelegate> objectSerializationDelegates;
        private Dictionary<Type, DeserializeObjectDelegate> objectDeserializationDelegates;

        private Dictionary<Type, SerializeFileDelegate> fileSerializationDelegates;
        private Dictionary<Type, DeserializeFileDelegate> fileDeserializationDelegates;

        private Dictionary<Type, SerializeFolderDelegate> folderSerializationDelegates;
        private Dictionary<Type, DeserializeFolderDelegate> folderDeserializationDelegates;

        private Dictionary<Type, Dictionary<Type, SerializeObjectDelegate>> objectSerializationSubDelegates;
        private Dictionary<Type, Dictionary<Type, DeserializeObjectDelegate>> objectDeserializationSubDelegates;

        private Dictionary<Type, Dictionary<Type, SerializeFileDelegate>> fileSerializationSubDelegates;
        private Dictionary<Type, Dictionary<Type, DeserializeFileDelegate>> fileDeserializationSubDelegates;

        private Dictionary<Type, Dictionary<Type, SerializeFolderDelegate>> folderSerializationSubDelegates;
        private Dictionary<Type, Dictionary<Type, DeserializeFolderDelegate>> folderDeserializationSubDelegates;
        #endregion

        #region Constructors
        public SerializationManager() : base()
        {
            serializabilityCache = new Dictionary<Type, Serializability>();

            primitiveSerializationDelegates = new Dictionary<Type, SerializePrimitiveDelegate>();
            primitiveDeserializationDelegates = new Dictionary<Type, DeserializePrimitiveDelegate>();

            objectSerializationDelegates = new Dictionary<Type, SerializeObjectDelegate>();
            objectDeserializationDelegates = new Dictionary<Type, DeserializeObjectDelegate>();

            fileSerializationDelegates = new Dictionary<Type, SerializeFileDelegate>();
            fileDeserializationDelegates = new Dictionary<Type, DeserializeFileDelegate>();

            folderSerializationDelegates = new Dictionary<Type, SerializeFolderDelegate>();
            folderDeserializationDelegates = new Dictionary<Type, DeserializeFolderDelegate>();

            objectSerializationSubDelegates = new Dictionary<Type, Dictionary<Type, SerializeObjectDelegate>>();
            objectDeserializationSubDelegates = new Dictionary<Type, Dictionary<Type, DeserializeObjectDelegate>>();

            fileSerializationSubDelegates = new Dictionary<Type, Dictionary<Type, SerializeFileDelegate>>();
            fileDeserializationSubDelegates = new Dictionary<Type, Dictionary<Type, DeserializeFileDelegate>>();

            folderSerializationSubDelegates = new Dictionary<Type, Dictionary<Type, SerializeFolderDelegate>>();
            folderDeserializationSubDelegates = new Dictionary<Type, Dictionary<Type, DeserializeFolderDelegate>>();

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

                #region Primitive (De-)serialization delegates registration
                Type boolType = typeof(bool);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!bool.TryParse(serializedPrimitive.Value, out bool boolValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a bool!");
                    }

                    primitive = boolValue;
                });

                Type byteType = typeof(byte);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!byte.TryParse(serializedPrimitive.Value, out byte byteValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a byte!");
                    }

                    primitive = byteValue;
                });

                Type sbyteType = typeof(sbyte);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!sbyte.TryParse(serializedPrimitive.Value, out sbyte sbyteValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as an sbyte!");
                    }

                    primitive = sbyteValue;
                });

                Type charType = typeof(char);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!char.TryParse(serializedPrimitive.Value, out char charValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a char!");
                    }

                    primitive = charValue;
                });

                Type decimalType = typeof(decimal);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!decimal.TryParse(serializedPrimitive.Value, out decimal decimalValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a decimal!");
                    }

                    primitive = decimalValue;
                });

                Type doubleType = typeof(double);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!double.TryParse(serializedPrimitive.Value, out double doubleValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a double!");
                    }

                    primitive = doubleValue;
                });

                Type floatType = typeof(float);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!float.TryParse(serializedPrimitive.Value, out float floatValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a float!");
                    }

                    primitive = floatValue;
                });

                Type intType = typeof(int);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!int.TryParse(serializedPrimitive.Value, out int intValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as an int!");
                    }

                    primitive = intValue;
                });

                Type uintType = typeof(uint);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!uint.TryParse(serializedPrimitive.Value, out uint uintValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a uint!");
                    }

                    primitive = uintValue;
                });

                Type longType = typeof(long);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!long.TryParse(serializedPrimitive.Value, out long longValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a long!");
                    }

                    primitive = longValue;
                });

                Type ulongType = typeof(ulong);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!ulong.TryParse(serializedPrimitive.Value, out ulong ulongValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a ulong!");
                    }

                    primitive = ulongValue;
                });

                Type shortType = typeof(short);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!short.TryParse(serializedPrimitive.Value, out short shortValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a short!");
                    }

                    primitive = shortValue;
                });

                Type ushortType = typeof(ushort);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!ushort.TryParse(serializedPrimitive.Value, out ushort ushortValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a ushort!");
                    }

                    primitive = ushortValue;
                });

                Type stringType = typeof(string);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    primitive = serializedPrimitive.Value;
                });

                Type guidType = typeof(Guid);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!Guid.TryParse(serializedPrimitive.Value, out Guid guidValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a Guid!");
                    }

                    primitive = guidValue;
                });

                Type bigIntType = typeof(BigInteger);
                primitiveSerializationDelegates.Add(boolType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(boolType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    if (!BigInteger.TryParse(serializedPrimitive.Value, out BigInteger bigIntValue))
                    {
                        throw new ArgumentException($"Attribute '{serializedPrimitive.Name}' with value '{serializedPrimitive.Value}' could not be parsed as a BigInteger!");
                    }

                    primitive = bigIntValue;
                });
                #endregion

                #region Type analysis
                Stopwatch stopwatch = new Stopwatch();
                stopwatch.Start();

                IEnumerable<Assembly> allAssemblies = AppDomain.CurrentDomain.GetAssemblies();
                IEnumerable<Type> allTypes = allAssemblies.SelectMany(assembly => assembly.GetTypes());
                List<Type> allSerializablePrimitiveTypes = new List<Type>();
                List<Type> allSerializableObjectTypes = new List<Type>();
                List<Type> allSerializableFileTypes = new List<Type>();
                List<Type> allSerializableFolderTypes = new List<Type>();

                foreach (Type type in allTypes)
                {
                    CacheSerializability(type, out Serializability serializability);
                    switch (serializability)
                    {
                        case Serializability.Object:
                            allSerializableObjectTypes.Add(type);
                            break;
                        case Serializability.File:
                            allSerializableFileTypes.Add(type);
                            break;
                        case Serializability.Folder:
                            allSerializableFolderTypes.Add(type);
                            break;
                    }
                }

                foreach (Type serializableObjectType in allSerializableObjectTypes)
                {
                    RegisterObjectSerializationDelegates(serializableObjectType);
                }
                foreach (Type serializableFileType in allSerializableFileTypes)
                {
                    RegisterFileSerializationDelegates(serializableFileType);
                }
                foreach (Type serializableFolderType in allSerializableFolderTypes)
                {
                    RegisterFolderSerializationDelegates(serializableFolderType);
                }

                foreach (Type serializableObjectType in allSerializableObjectTypes)
                {
                    RegisterObjectSerializationSubDelegates(serializableObjectType);
                }
                foreach (Type serializableFileType in allSerializableFileTypes)
                {
                    RegisterFileSerializationSubDelegates(serializableFileType);
                }
                foreach (Type serializableFolderType in allSerializableFolderTypes)
                {
                    RegisterFolderSerializationSubDelegates(serializableFolderType);
                }

                stopwatch.Stop();
                UnityEngine.Debug.Log($"Caching {allTypes.Count()} type's serialization information for {allAssemblies.Count()} assemblies took {stopwatch.ElapsedMilliseconds}ms");
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
        public Serializability GetSerializability(Type type)
        {
            return serializabilityCache[type];
        }

        private void CacheSerializability(Type type, out Serializability serializability)
        {
            if (type == null)
            {
                throw new ArgumentNullException(nameof(type));
            }

            if (serializabilityCache.ContainsKey(type))
            {
                throw new InvalidOperationException($"Serializability already cached for type '{type}'!");
            }

            serializability = Serializability.None;
            
            if (type.IsPublic && !type.IsAbstract && (type.IsClass || type.IsValueType || type.IsEnum) && type.GetConstructor(Type.EmptyTypes) != null)
            {
                List<Serializability> detectedSerializabilities = new List<Serializability>();

                if (IsPrimitiveTypeSerializationDelegateRegistered(type) && IsPrimitiveTypeDeserializationDelegateRegistered(type))
                {
                    detectedSerializabilities.Add(Serializability.Primitive);
                }
                if (type.IsDefined(typeof(SerializableFolderAttribute), false))
                {
                    detectedSerializabilities.Add(Serializability.Folder);
                }
                if (type.IsDefined(typeof(SerializableFileAttribute), false))
                {
                    detectedSerializabilities.Add(Serializability.File);
                }
                if (type.IsDefined(typeof(SerializableObjectAttribute), false))
                {
                    detectedSerializabilities.Add(Serializability.Object);
                }

                if (detectedSerializabilities.Count > 1)
                {
                    throw new InvalidOperationException($"Type {type.FullName} has conflicting serializability!");
                }
                else if (detectedSerializabilities.Count == 1)
                {
                    serializability = detectedSerializabilities[0];
                }
                
                serializabilityCache.Add(type, serializability);
            }
            else
            {
                serializabilityCache.Add(type, Serializability.None);
            }
        }

        private bool IsPrimitiveTypeSerializationDelegateRegistered(Type serializablePrimitiveType)
        {
            return primitiveSerializationDelegates.ContainsKey(serializablePrimitiveType);
        }
        private bool IsPrimitiveTypeDeserializationDelegateRegistered(Type serializablePrimitiveType)
        {
            return primitiveDeserializationDelegates.ContainsKey(serializablePrimitiveType);
        }

        private bool IsObjectTypeSerializationDelegateRegistered(Type serializableObjectType)
        {
            return objectSerializationDelegates.ContainsKey(serializableObjectType);
        }
        private bool IsObjectTypeDeserializationDelegateRegistered(Type serializableObjectType)
        {
            return objectDeserializationDelegates.ContainsKey(serializableObjectType);
        }

        private bool IsFileTypeSerializationDelegateRegistered(Type serializableFileType)
        {
            return objectSerializationDelegates.ContainsKey(serializableFileType);
        }
        private bool IsFileTypeDeserializationDelegateRegistered(Type serializableFileType)
        {
            return objectDeserializationDelegates.ContainsKey(serializableFileType);
        }

        private bool IsFolderTypeSerializationDelegateRegistered(Type serializableFolderType)
        {
            return objectSerializationDelegates.ContainsKey(serializableFolderType);
        }
        private bool IsFolderTypeDeserializationDelegateRegistered(Type serializableFolderType)
        {
            return objectDeserializationDelegates.ContainsKey(serializableFolderType);
        }

        // TODO: Also cache the default (de-)serialization delegates for object, file and folder
        // TODO: Also cache the needed "sub-serializers" for each type
        private void RegisterObjectSerializationDelegates(Type serializableObjectType)
        {
            SerializableObjectAttribute serializableObjectAttribute = serializableObjectType.GetCustomAttribute<SerializableObjectAttribute>();

            bool overrideSerialization = serializableObjectAttribute.OverrideSerialization;
            bool overrideDeserialization = serializableObjectAttribute.OverrideDeserialization;

            if (overrideSerialization)
            {
                MethodInfo methodInfo = serializableObjectType.GetMethod("Serialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(string), typeof(object), typeof(XElement).MakeByRefType() }, null);

                if (methodInfo == null)
                {
                    throw new Exception($"Type '{serializableObjectType}' is marked as overriding the defualt serialization behaviour, but it does not implement a method with the signature 'public static void Serialize(string listName, object _object, out XElement serializedList)'!");
                }

                objectSerializationDelegates.Add(serializableObjectType, (SerializeObjectDelegate)methodInfo.CreateDelegate(typeof(SerializeObjectDelegate)))
            }
            if (overrideDeserialization)
            {
                MethodInfo methodInfo = serializableObjectType.GetMethod("Deserialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(XElement), typeof(object).MakeByRefType() }, null);

                if (methodInfo == null)
                {
                    throw new Exception($"Type '{serializableObjectType}' is marked as overriding the defualt deserialization behaviour, but it does not implement a method with the signature 'public static void Deserialize(XElement serializedList, out object _object)'!");
                }

                objectDeserializationDelegates.Add(serializableObjectType, (DeserializeObjectDelegate)methodInfo.CreateDelegate(typeof(DeserializeObjectDelegate)));
            }
            if (overrideSerialization && overrideDeserialization)
            {
                return;
            }

            PropertyInfo[] properties = serializableObjectType.GetProperties();
            FieldInfo[] fields = serializableObjectType.GetFields();

            Dictionary<string, SerializePrimitiveDelegate> serializePrimitivePropertyDelegates = new Dictionary<string, SerializePrimitiveDelegate>();
            Dictionary<string, DeserializePrimitiveDelegate> deserializePrimitivePropertyDelegates = new Dictionary<string, DeserializePrimitiveDelegate>();

            Dictionary<string, SerializeObjectDelegate> serializeObjectPropertyDelegates = new Dictionary<string, SerializeObjectDelegate>();
            Dictionary<string, DeserializeObjectDelegate> deserializeObjectPropertyDelegates = new Dictionary<string, DeserializeObjectDelegate>();

            Dictionary<string, SerializePrimitiveDelegate> serializePrimitiveFieldDelegates = new Dictionary<string, SerializePrimitiveDelegate>();
            Dictionary<string, DeserializePrimitiveDelegate> deserializePrimitiveFieldDelegates = new Dictionary<string, DeserializePrimitiveDelegate>();

            Dictionary<string, SerializeObjectDelegate> serializeObjectFieldDelegates = new Dictionary<string, SerializeObjectDelegate>();
            Dictionary<string, DeserializeObjectDelegate> deserializeObjectFieldDelegates = new Dictionary<string, DeserializeObjectDelegate>();

            for (int i = 0; i < properties.Length; i++)
            {
                PropertyInfo property = properties[i];
                Type propertyType = property.PropertyType;
                Serializability propertySerializability = GetSerializability(propertyType);
                switch (propertySerializability)
                {
                    case Serializability.Primitive:
                        if (!overrideSerialization)
                        {
                            if (!IsPrimitiveTypeSerializationDelegateRegistered(propertyType))
                            {
                                throw new Exception($"No primitive serialization delegate registered for type '{propertyType}'!"
                            }
                            
                            serializePrimitivePropertyDelegates.Add(property.Name, primitiveSerializationDelegates[propertyType]);
                        }
                        if (!overrideDeserialization)
                        {
                            if (!IsPrimitiveTypeDeserializationDelegateRegistered(propertyType))
                            {
                                throw new Exception($"No primitive deserialization delegate registered for type '{propertyType}'!"
                            }
                            
                            deserializePrimitivePropertyDelegates.Add(property.Name, primitiveDeserializationDelegates[propertyType]);
                        }
                        break;
                    case Serializability.Object:
                        if (!overrideSerialization)
                        {
                            if (!IsObjectTypeSerializationDelegateRegistered(propertyType))
                            {
                                RegisterObjectSerializationDelegates(propertyType); // TODO: THIS WILL BREAK WITH SELF-REFERENCING OBJECTS
                            }
                            
                            serializeObjectPropertyDelegates.Add(property.Name, objectSerializationDelegates[propertyType]);
                        }
                        if (!overrideDeserialization)
                        {
                            if (!IsObjectTypeDeserializationDelegateRegistered(propertyType))
                            {
                                RegisterObjectSerializationDelegates(propertyType); // TODO: THIS WILL BREAK WITH SELF-REFERENCING OBJECTS
                            }

                            deserializeObjectPropertyDelegates.Add(property.Name, objectDeserializationDelegates[propertyType]);
                        }
                        break;
                    case Serializability.File:
                        throw new Exception($"Properties of serializable objects may not be files!");
                    case Serializability.Folder:
                        throw new Exception($"Properties of serializable objects may not be folders!");
                }
            }

            for (int i = 0; i < fields.Length; i++)
            {
                FieldInfo field = fields[i];
                Type fieldType = field.FieldType;
                Serializability fieldSerializability = GetSerializability(fieldType);
                switch (fieldSerializability)
                {
                    case Serializability.Primitive:
                        if (!overrideSerialization)
                        {
                            if (!IsPrimitiveTypeSerializationDelegateRegistered(fieldType))
                            {
                                throw new Exception($"No primitive serialization delegate registered for type '{fieldType}'!"
                            }
                            serializePrimitiveFieldDelegates.Add(field.Name, primitiveSerializationDelegates[fieldType]);
                        }
                        if (!overrideDeserialization)
                        {
                            if (!IsPrimitiveTypeDeserializationDelegateRegistered(fieldType))
                            {
                                throw new Exception($"No primitive deserialization delegate registered for type '{fieldType}'!"
                            }
                            deserializePrimitiveFieldDelegates.Add(field.Name, primitiveDeserializationDelegates[fieldType]);
                        }
                        break;
                    case Serializability.Object:
                        if (!overrideSerialization)
                        {
                            if (!IsObjectTypeSerializationDelegateRegistered(fieldType))
                            {
                                RegisterObjectSerializationDelegates(fieldType); // TODO: THIS WILL BREAK WITH SELF-REFERENCING OBJECTS
                            }

                            serializeObjectFieldDelegates.Add(field.Name, objectSerializationDelegates[fieldType]);
                        }
                        if (!overrideDeserialization)
                        {
                            if (!IsObjectTypeDeserializationDelegateRegistered(fieldType))
                            {
                                RegisterObjectSerializationDelegates(fieldType); // TODO: THIS WILL BREAK WITH SELF-REFERENCING OBJECTS
                            }

                            deserializeObjectFieldDelegates.Add(field.Name, objectDeserializationDelegates[fieldType]);
                        }
                        break;
                    case Serializability.File:
                        throw new Exception($"Fields of serializable objects may not be files!");
                    case Serializability.Folder:
                        throw new Exception($"Fields of serializable objects may not be folders!");
                }
            }

            if (!overrideSerialization)
            {
                objectSerializationDelegates.Add(serializableObjectType, (string objectName, object _object, out XElement serializedObject) =>
                {
                    serializedObject = new XElement(objectName);

                    // get the object's properties and fields

                    // serialize each primitive property via the primitive property serialization delegate which it is mapped to via it's name in serializePrimitivePropertyDelegates
                    // serialize each object property via the object property serialization delegate which it is mapped to via it's name in serializeObjectPropertyDelegates

                    // serialize each primitive field via the primitive field serialization delegate which it is mapped to via it's name in serializePrimitiveFieldDelegates
                    // serialize each object property via the object field serialization delegate which it is mapped to via it's name in serializeObjectFieldDelegates
                })
            }
            if (!overrideDeserialization)
            {
                objectDeserializationDelegates.Add(serializableObjectType, (XElement serializedObject, out object _object) =>
                {
                    _object = Activator.CreateInstance(serializableObjectType);

                    // search and find the object's properties and fields which are mentioned in the dictionaries deserializePrimitivePropertyDelegates, deserializeObjectPropertyDelegates, deserializePrimitiveFieldDelegates, deserializeObjectFieldDelegates
                    // if any property or field is missing, throw an exception
                    // else, deserialize the properties and fields via the delegates which they are mapped to in the dictionaries via their name
                });
            }
        }
        private void RegisterFileSerializationDelegates(Type serializableFileType)
        {
        }
        private void RegisterFolderSerializationDelegates(Type serializableFolderType)
        {
        }

        private void RegisterObjectSerializationSubDelegates(Type serializableObjectType)
        {

        }
        private void RegisterFileSerializationSubDelegates(Type serializableFileType)
        {

        }
        private void RegisterFolderSerializationSubDelegates(Type serializableFolderType)
        {

        }
        #endregion
    }
}
