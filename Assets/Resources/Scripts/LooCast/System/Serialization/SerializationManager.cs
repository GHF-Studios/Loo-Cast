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

                if (IsPrimitiveTypeSerializationDelegateCached(type) && IsPrimitiveTypeDeserializationDelegateCached(type))
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

        private bool IsPrimitiveTypeSerializationDelegateCached(Type serializablePrimitiveType)
        {
            return primitiveSerializationDelegates.ContainsKey(serializablePrimitiveType);
        }
        private bool IsPrimitiveTypeDeserializationDelegateCached(Type serializablePrimitiveType)
        {
            return primitiveDeserializationDelegates.ContainsKey(serializablePrimitiveType);
        }

        private bool IsObjectTypeSerializationDelegateCached(Type serializableObjectType)
        {
            return objectSerializationDelegates.ContainsKey(serializableObjectType);
        }
        private bool IsObjectTypeDeserializationDelegateCached(Type serializableObjectType)
        {
            return objectDeserializationDelegates.ContainsKey(serializableObjectType);
        }

        private bool IsFileTypeSerializationDelegateCached(Type serializableFileType)
        {
            return objectSerializationDelegates.ContainsKey(serializableFileType);
        }
        private bool IsFileTypeDeserializationDelegateCached(Type serializableFileType)
        {
            return objectDeserializationDelegates.ContainsKey(serializableFileType);
        }

        private bool IsFolderTypeSerializationDelegateCached(Type serializableFolderType)
        {
            return objectSerializationDelegates.ContainsKey(serializableFolderType);
        }
        private bool IsFolderTypeDeserializationDelegateCached(Type serializableFolderType)
        {
            return objectDeserializationDelegates.ContainsKey(serializableFolderType);
        }

        // TODO: Also cache the default (de-)serialization delegates for object, file and folder
        // TODO: Also cache the needed "sub-serializers" for each type
        private void RegisterObjectSerializationDelegates(Type serializableObjectType)
        {
            PropertyInfo[] properties = serializableObjectType.GetProperties();
            FieldInfo[] fields = serializableObjectType.GetFields();

            List<SerializePrimitiveDelegate> serializePrimitiveDelegates = new List<SerializePrimitiveDelegate>();
            List<DeserializePrimitiveDelegate> deserializePrimitiveDelegates = new List<DeserializePrimitiveDelegate>();

            List<SerializeObjectDelegate> serializeObjectDelegates = new List<SerializeObjectDelegate>();
            List<DeserializeObjectDelegate> deserializeObjectDelegates = new List<DeserializeObjectDelegate>();

            for (int i = 0; i < properties.Length; i++)
            {
                PropertyInfo property = properties[i];
                Type propertyType = property.PropertyType;
                Serializability propertySerializability = GetSerializability(propertyType);
                switch (propertySerializability)
                {
                    case Serializability.Object:
                        if (!IsObjectTypeSerializationDelegateCached(propertyType))
                        {
                            if (false/*TODO: if propertyType is marked as override serialization*/)
                            {

                            }
                            else
                            {
                                serializeObjectDelegates.Add((string objectName, object _object, out XElement serializedObject) =>
                                {
                                    // Get all properties and fields of the object
                                    // Get all property and field types
                                    // Locally cache all necessary serializers for the property and field types
                                    // Loop through all properties and serialize them
                                    // Loop through all fields and serialize them
                                });
                            }
                        }
                        if (!IsObjectTypeDeserializationDelegateCached(propertyType))
                        {
                            if (false/*TODO: if propertyType is marked as override deserialization*/)
                            {

                            }
                            else
                            {
                                deserializeObjectDelegates.Add((XElement serializedObject, out object _object) =>
                                {
                                    // Get all properties and fields of the object
                                    // Get all property and field types
                                    // Locally cache all necessary serializers for the property and field types
                                    // Loop through all properties and serialize them
                                    // Loop through all fields and serialize them
                                });
                            }
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
                    case Serializability.Object:
                        if (!IsObjectTypeSerializationDelegateCached(fieldType))
                        {
                            if (false/*TODO: if fieldType is marked as override serialization*/)
                            {

                            }
                            else
                            {
                                serializeObjectDelegates.Add((string objectName, object _object, out XElement serializedObject) =>
                                {
                                    // Get all properties and fields of the object
                                    // Get all property and field types
                                    // Locally cache all necessary serializers for the property and field types
                                    // Loop through all properties and serialize them
                                    // Loop through all fields and serialize them
                                });
                            }
                        }
                        if (!IsObjectTypeDeserializationDelegateCached(fieldType))
                        {
                            if (false/*TODO: if fieldType is marked as override deserialization*/)
                            {

                            }
                            else
                            {
                                deserializeObjectDelegates.Add((XElement serializedObject, out object _object) =>
                                {
                                    // Get all properties and fields of the object
                                    // Get all property and field types
                                    // Locally cache all necessary serializers for the property and field types
                                    // Loop through all properties and serialize them
                                    // Loop through all fields and serialize them
                                });
                            }
                        }
                        break;
                    case Serializability.File:
                        throw new Exception($"Fields of serializable objects may not be files!");
                    case Serializability.Folder:
                        throw new Exception($"Fields of serializable objects may not be folders!");
                }
            }
        }

        private void RegisterFileSerializationDelegates(Type serializableFileType)
        {

        }

        private void RegisterFolderSerializationDelegates(Type serializableFolderType)
        {

        }
        #endregion
    }
}
