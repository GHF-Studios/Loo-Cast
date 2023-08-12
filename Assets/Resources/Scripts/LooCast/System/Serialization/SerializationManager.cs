using System;
using System.IO;
using System.Reflection;
using System.Linq;
using System.Collections.Generic;
using System.Diagnostics;
using System.Xml.Linq;
using System.Numerics;
using UnityEngine;

namespace LooCast.System.Serialization
{
    using LooCast.System.ECS;
    using UnityEditor.ShaderGraph.Internal;
    using static LooCast.System.Serialization.OldSerializationManager;

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
        private HashSet<Type> registeredUnserializableTypes;
        private Dictionary<Type, PrimitiveTypeInfo> registeredPrimitiveTypeInfos;
        private Dictionary<Type, NonGenericObjectTypeInfo> registeredNonGenericObjectTypeInfos;
        private Dictionary<Type, GenericObjectTypeInfo> registeredGenericObjectTypeInfos;
        private Dictionary<Type, FileTypeInfo> registeredFileTypeInfos;
        private Dictionary<Type, FolderTypeInfo> registeredFolderTypeInfos;

        private Dictionary<Type, NonGenericObjectTypeInfo> newlyRegisteredNonGenericObjectTypeInfos;
        private Dictionary<Type, GenericObjectTypeInfo> newlyRegisteredGenericObjectTypeInfos;
        private Dictionary<Type, FileTypeInfo> newlyRegisteredFileTypeInfos;
        private Dictionary<Type, FolderTypeInfo> newlyRegisteredFolderTypeInfos;
        #endregion

        #region Constructors
        public SerializationManager() : base()
        {
            registeredUnserializableTypes = new HashSet<Type>();
            registeredPrimitiveTypeInfos = new Dictionary<Type, PrimitiveTypeInfo>();
            registeredNonGenericObjectTypeInfos = new Dictionary<Type, NonGenericObjectTypeInfo>();
            registeredGenericObjectTypeInfos = new Dictionary<Type, GenericObjectTypeInfo>();
            registeredFileTypeInfos = new Dictionary<Type, FileTypeInfo>();
            registeredFolderTypeInfos = new Dictionary<Type, FolderTypeInfo>();

            newlyRegisteredNonGenericObjectTypeInfos = new Dictionary<Type, NonGenericObjectTypeInfo>();
            newlyRegisteredGenericObjectTypeInfos = new Dictionary<Type, GenericObjectTypeInfo>();
            newlyRegisteredFileTypeInfos = new Dictionary<Type, FileTypeInfo>();
            newlyRegisteredFolderTypeInfos = new Dictionary<Type, FolderTypeInfo>();

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
                PrimitiveTypeInfo.Serialize boolSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize boolDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(boolType, new PrimitiveTypeInfo(boolType, boolSerializeDelegate, boolDeserializeDelegate));

                Type byteType = typeof(byte);
                PrimitiveTypeInfo.Serialize byteSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize byteDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(byteType, new PrimitiveTypeInfo(byteType, byteSerializeDelegate, byteDeserializeDelegate));

                Type sbyteType = typeof(sbyte);
                PrimitiveTypeInfo.Serialize sbyteSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize sbyteDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(sbyteType, new PrimitiveTypeInfo(sbyteType, sbyteSerializeDelegate, sbyteDeserializeDelegate));

                Type charType = typeof(char);
                PrimitiveTypeInfo.Serialize charSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize charDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(charType, new PrimitiveTypeInfo(charType, charSerializeDelegate, charDeserializeDelegate));

                Type decimalType = typeof(decimal);
                PrimitiveTypeInfo.Serialize decimalSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize decimalDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(decimalType, new PrimitiveTypeInfo(decimalType, decimalSerializeDelegate, decimalDeserializeDelegate));

                Type doubleType = typeof(double);
                PrimitiveTypeInfo.Serialize doubleSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize doubleDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(doubleType, new PrimitiveTypeInfo(doubleType, doubleSerializeDelegate, doubleDeserializeDelegate));

                Type floatType = typeof(float);
                PrimitiveTypeInfo.Serialize floatSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize floatDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(floatType, new PrimitiveTypeInfo(floatType, floatSerializeDelegate, floatDeserializeDelegate));

                Type intType = typeof(int);
                PrimitiveTypeInfo.Serialize intSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize intDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(intType, new PrimitiveTypeInfo(intType, intSerializeDelegate, intDeserializeDelegate));

                Type uintType = typeof(uint);
                PrimitiveTypeInfo.Serialize uintSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize uintDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(uintType, new PrimitiveTypeInfo(uintType, uintSerializeDelegate, uintDeserializeDelegate));

                Type longType = typeof(long);
                PrimitiveTypeInfo.Serialize longSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize longDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(longType, new PrimitiveTypeInfo(longType, longSerializeDelegate, longDeserializeDelegate));

                Type ulongType = typeof(ulong);
                PrimitiveTypeInfo.Serialize ulongSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize ulongDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(ulongType, new PrimitiveTypeInfo(ulongType, ulongSerializeDelegate, ulongDeserializeDelegate));

                Type shortType = typeof(short);
                PrimitiveTypeInfo.Serialize shortSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize shortDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(shortType, new PrimitiveTypeInfo(shortType, shortSerializeDelegate, shortDeserializeDelegate));

                Type ushortType = typeof(ushort);
                PrimitiveTypeInfo.Serialize ushortSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize ushortDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(ushortType, new PrimitiveTypeInfo(ushortType, ushortSerializeDelegate, ushortDeserializeDelegate));

                Type stringType = typeof(string);
                PrimitiveTypeInfo.Serialize stringSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize stringDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    primitive = serializedPrimitive.Value;
                };
                registeredPrimitiveTypeInfos.Add(stringType, new PrimitiveTypeInfo(stringType, stringSerializeDelegate, stringDeserializeDelegate));

                Type guidType = typeof(Guid);
                PrimitiveTypeInfo.Serialize guidSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize guidDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(guidType, new PrimitiveTypeInfo(guidType, guidSerializeDelegate, guidDeserializeDelegate));

                Type bigIntType = typeof(BigInteger);
                PrimitiveTypeInfo.Serialize bigIntSerializeDelegate = (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                };
                PrimitiveTypeInfo.Deserialize bigIntDeserializeDelegate = (XAttribute serializedPrimitive, out object primitive) =>
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
                };
                registeredPrimitiveTypeInfos.Add(bigIntType, new PrimitiveTypeInfo(bigIntType, bigIntSerializeDelegate, bigIntDeserializeDelegate));
                #endregion

                #region Type caching
                Stopwatch stopwatch = new Stopwatch();
                stopwatch.Start();

                IEnumerable<Assembly> allInitialAssemblyDefinitions = AppDomain.CurrentDomain.GetAssemblies();
                IEnumerable<Type> allInitialTypeDefinitions = allInitialAssemblyDefinitions.SelectMany(assembly => assembly.GetTypes());

                RegisterTypes(allInitialTypeDefinitions);

                stopwatch.Stop();
                int cachedTypeCount = registeredUnserializableTypes.Count + registeredPrimitiveTypeInfos.Count + newlyRegisteredNonGenericObjectTypeInfos.Count + newlyRegisteredGenericObjectTypeInfos.Count + newlyRegisteredFileTypeInfos.Count + newlyRegisteredFolderTypeInfos.Count;
                UnityEngine.Debug.Log($"Analyzing {cachedTypeCount} types took {stopwatch.ElapsedMilliseconds}ms");
                #endregion

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
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

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
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
        /// <summary>
        /// For performance reasons it is highly recommended to register as many types as possible at once!
        /// </summary>
        public void RegisterTypes(IEnumerable<Type> types)
        {
            foreach (Type type in types)
            {
                AnalyzeType(type);
            }

            IEnumerable<ObjectTypeInfo> newlyRegisteredObjectTypeInfos = newlyRegisteredNonGenericObjectTypeInfos.Values.Cast<ObjectTypeInfo>().Concat(newlyRegisteredGenericObjectTypeInfos.Values);
            HashSet<ObjectTypeInfo>[] sortedNewlyRegisteredObjectTypeInfoSets = TopologicallySortObjectTypeInfos(newlyRegisteredObjectTypeInfos);
            HashSet<FolderTypeInfo>[] sortedNewlyRegisteredFolderTypeInfoSets = TopologicallySortFolderTypeInfos(newlyRegisteredFolderTypeInfos.Values);

            foreach (HashSet<ObjectTypeInfo> sortedNewlyRegisteredObjectTypeInfoSet in sortedNewlyRegisteredObjectTypeInfoSets)
            {
                foreach (ObjectTypeInfo objectTypeInfo in sortedNewlyRegisteredObjectTypeInfoSet)
                {
                    if (objectTypeInfo.Serializability == Serializability.NonGenericObject)
                    {
                        NonGenericObjectTypeInfo nonGenericObjectTypeInfo = (NonGenericObjectTypeInfo)objectTypeInfo;
                        ComposeNonGenericObjectTypeDelegates(nonGenericObjectTypeInfo);
                        registeredNonGenericObjectTypeInfos.Add(objectTypeInfo.Type, nonGenericObjectTypeInfo);
                    }
                    else if (objectTypeInfo.Serializability == Serializability.GenericObject)
                    {
                        GenericObjectTypeInfo genericObjectTypeInfo = (GenericObjectTypeInfo)objectTypeInfo;
                        registeredGenericObjectTypeInfos.Add(objectTypeInfo.Type, genericObjectTypeInfo);
                    }
                }
            }

            foreach (FileTypeInfo fileTypeInfo in newlyRegisteredFileTypeInfos.Values)
            {
                ComposeFileTypeDelegates(fileTypeInfo);
                registeredFileTypeInfos.Add(fileTypeInfo.Type, fileTypeInfo);
            }

            foreach (HashSet<FolderTypeInfo> sortedNewlyRegisteredFolderTypeInfoSet in sortedNewlyRegisteredFolderTypeInfoSets)
            {
                foreach (FolderTypeInfo folderTypeInfo in sortedNewlyRegisteredFolderTypeInfoSet)
                {
                    ComposeFolderTypeDelegates(folderTypeInfo);
                    registeredFolderTypeInfos.Add(folderTypeInfo.Type, folderTypeInfo);
                }
            }

            newlyRegisteredNonGenericObjectTypeInfos.Clear();
            newlyRegisteredGenericObjectTypeInfos.Clear();
            newlyRegisteredFileTypeInfos.Clear();
            newlyRegisteredFolderTypeInfos.Clear();
        }

        /// <summary>
        /// For performance reasons it is highly recommended to register as many types as possible at once!
        /// </summary>
        public void UnregisterTypes(IEnumerable<Type> types)
        {
            
        }

        private TypeInfo AnalyzeType(Type type)
        {
            if (registeredUnserializableTypes.Contains(type))
            {
                return null;
            }
            if (registeredPrimitiveTypeInfos.TryGetValue(type, out PrimitiveTypeInfo _primitiveTypeInfo))
            {
                return _primitiveTypeInfo;
            }
            if (registeredNonGenericObjectTypeInfos.TryGetValue(type, out NonGenericObjectTypeInfo _nonGenericObjectTypeInfo))
            {
                return _nonGenericObjectTypeInfo;
            }
            if (registeredGenericObjectTypeInfos.TryGetValue(type, out GenericObjectTypeInfo _genericObjectTypeInfo))
            {
                return _genericObjectTypeInfo;
            }
            if (registeredFileTypeInfos.TryGetValue(type, out FileTypeInfo _fileTypeInfo))
            {
                return _fileTypeInfo;
            }
            if (registeredFolderTypeInfos.TryGetValue(type, out FolderTypeInfo _folderTypeInfo))
            {
                return _folderTypeInfo;
            }
            if (newlyRegisteredNonGenericObjectTypeInfos.TryGetValue(type, out NonGenericObjectTypeInfo _newNonGenericObjectTypeInfo))
            {
                return _newNonGenericObjectTypeInfo;
            }
            if (newlyRegisteredGenericObjectTypeInfos.TryGetValue(type, out GenericObjectTypeInfo _newGenericObjectTypeInfo))
            {
                return _newGenericObjectTypeInfo;
            }
            if (newlyRegisteredFileTypeInfos.TryGetValue(type, out FileTypeInfo _newFileTypeInfo))
            {
                return _newFileTypeInfo;
            }
            if (newlyRegisteredFolderTypeInfos.TryGetValue(type, out FolderTypeInfo _newFolderTypeInfo))
            {
                return _newFolderTypeInfo;
            }

            if (type.IsGenericTypeDefinition)
            {
                registeredUnserializableTypes.Add(type);
                return null;
            }
            if (type.IsAbstract)
            {
                registeredUnserializableTypes.Add(type);
                return null;
            }
            if (!type.IsPublic && !type.IsNestedPublic)
            {
                registeredUnserializableTypes.Add(type);
                return null;
            }
            if (!type.IsClass && !type.IsValueType && !type.IsEnum)
            {
                registeredUnserializableTypes.Add(type);
                return null;
            }

            SerializableNonGenericObjectAttribute serializableNonGenericObjectAttribute = type.GetCustomAttribute<SerializableNonGenericObjectAttribute>(false);
            SerializableGenericObjectAttribute serializableGenericObjectAttribute = type.GetCustomAttribute<SerializableGenericObjectAttribute>(false);
            SerializableFileAttribute serializableFileAttribute = type.GetCustomAttribute<SerializableFileAttribute>(false);
            SerializableFolderAttribute serializableFolderAttribute = type.GetCustomAttribute<SerializableFolderAttribute>(false);

            if (serializableNonGenericObjectAttribute == null && serializableGenericObjectAttribute == null && serializableFileAttribute == null && serializableFolderAttribute == null)
            {
                registeredUnserializableTypes.Add(type);
                return null;
            }
            if (!(serializableNonGenericObjectAttribute != null ^ serializableGenericObjectAttribute != null ^ serializableFileAttribute != null ^ serializableFolderAttribute != null))
            {
                throw new Exception($"Type '{type.FullName}' is marked as more than one serializable type!");
            }

            if (serializableNonGenericObjectAttribute != null)
            {
                NonGenericObjectTypeInfo nonGenericObjectTypeInfo = new NonGenericObjectTypeInfo(type);
                newlyRegisteredNonGenericObjectTypeInfos.Add(type, nonGenericObjectTypeInfo);
                
                nonGenericObjectTypeInfo.Properties = type.GetProperties(BindingFlags.Public | BindingFlags.Instance).Where((property) => property.GetGetMethod() != null && property.GetSetMethod() != null).ToArray();
                nonGenericObjectTypeInfo.Fields = type.GetFields(BindingFlags.Public | BindingFlags.Instance);

                nonGenericObjectTypeInfo.OverrideSerialization = serializableNonGenericObjectAttribute.OverrideSerialization;
                nonGenericObjectTypeInfo.OverrideDeserialization = serializableNonGenericObjectAttribute.OverrideDeserialization;

                if (nonGenericObjectTypeInfo.OverrideSerialization)
                {
                    MethodInfo methodInfo = nonGenericObjectTypeInfo.Type.GetMethod("Serialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(string), typeof(object), typeof(XElement).MakeByRefType() }, null);

                    if (methodInfo == null)
                    {
                        throw new Exception($"The non-generic object type '{nonGenericObjectTypeInfo.Type}' is marked as overriding the defualt serialization behaviour, but it does not implement a method with the signature 'public static void Serialize(string objectName, object _object, out XElement serializedObject)'!");
                    }

                    nonGenericObjectTypeInfo.SerializeDelegate = (NonGenericObjectTypeInfo.Serialize)methodInfo.CreateDelegate(typeof(NonGenericObjectTypeInfo.Serialize));
                }
                if (nonGenericObjectTypeInfo.OverrideDeserialization)
                {
                    MethodInfo methodInfo = nonGenericObjectTypeInfo.Type.GetMethod("Deserialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(XElement), typeof(object).MakeByRefType() }, null);

                    if (methodInfo == null)
                    {
                        throw new Exception($"The non-generic object type '{nonGenericObjectTypeInfo}' is marked as overriding the defualt deserialization behaviour, but it does not implement a method with the signature 'public static void Deserialize(XElement serializedObject, out object _object)'!");
                    }

                    nonGenericObjectTypeInfo.DeserializeDelegate = (NonGenericObjectTypeInfo.Deserialize)methodInfo.CreateDelegate(typeof(NonGenericObjectTypeInfo.Deserialize));
                }

                IEnumerable<Type> propertyTypes = nonGenericObjectTypeInfo.Properties.Select(property => property.PropertyType);
                IEnumerable<Type> fieldTypes = nonGenericObjectTypeInfo.Fields.Select(field => field.FieldType);
                IEnumerable<Type> utilizedTypes = propertyTypes.Concat(fieldTypes).Distinct();

                nonGenericObjectTypeInfo.PrimitiveTypeDependencies = new HashSet<PrimitiveTypeInfo>();
                nonGenericObjectTypeInfo.NonGenericObjectTypeDependencies = new HashSet<NonGenericObjectTypeInfo>();
                nonGenericObjectTypeInfo.GenericObjectTypeDependencies = new HashSet<GenericObjectTypeInfo>();

                foreach (Type utilizedType in utilizedTypes)
                {
                    AnalyzeType(utilizedType);
                    
                    if (registeredUnserializableTypes.Contains(utilizedType))
                    {
                        throw new Exception($"Non-Generic object type '{type.FullName}' utilizes unserializable type '{utilizedType.FullName}'! A non-generic object type cannot utilize an unserializable type!");
                    }
                    else if (registeredPrimitiveTypeInfos.ContainsKey(utilizedType))
                    {
                        nonGenericObjectTypeInfo.PrimitiveTypeDependencies.Add(registeredPrimitiveTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (newlyRegisteredNonGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        nonGenericObjectTypeInfo.NonGenericObjectTypeDependencies.Add(newlyRegisteredNonGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (registeredNonGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        nonGenericObjectTypeInfo.NonGenericObjectTypeDependencies.Add(registeredNonGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (newlyRegisteredGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        nonGenericObjectTypeInfo.GenericObjectTypeDependencies.Add(newlyRegisteredGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (registeredGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        nonGenericObjectTypeInfo.GenericObjectTypeDependencies.Add(registeredGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (newlyRegisteredFileTypeInfos.ContainsKey(utilizedType) || registeredFileTypeInfos.ContainsKey(utilizedType))
                    {
                        throw new Exception($"Non-Generic object type '{type.FullName}' utilizes file type '{utilizedType.FullName}'! A non-generic object type cannot utilize a file type!");
                    }
                    else if (newlyRegisteredFolderTypeInfos.ContainsKey(utilizedType) || registeredFolderTypeInfos.ContainsKey(utilizedType))
                    {
                        throw new Exception($"Non-Generic object type '{type.FullName}' utilizes folder type '{utilizedType.FullName}'! A non-generic object type cannot utilize a folder type!");
                    }
                }

                return nonGenericObjectTypeInfo;
            }
            else if (serializableGenericObjectAttribute != null)
            {
                GenericObjectTypeInfo genericObjectTypeInfo = new GenericObjectTypeInfo(type);
                newlyRegisteredGenericObjectTypeInfos.Add(type, genericObjectTypeInfo);

                MethodInfo serializeMethodInfo = genericObjectTypeInfo.Type.GetMethod("Serialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(string), typeof(object), typeof(XElement).MakeByRefType() }, null);
                if (serializeMethodInfo == null)
                {
                    throw new Exception($"The generic object type '{genericObjectTypeInfo.Type}' does not provide the mandatory method implementation with the signature 'public static void Serialize(string objectName, object _object, out XElement serializedObject)'!");
                }
                genericObjectTypeInfo.SerializeDelegate = (GenericObjectTypeInfo.Serialize)serializeMethodInfo.CreateDelegate(typeof(GenericObjectTypeInfo.Serialize));

                MethodInfo deserializeMethodInfo = genericObjectTypeInfo.Type.GetMethod("Deserialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(XElement), typeof(object).MakeByRefType() }, null);
                if (deserializeMethodInfo == null)
                {
                    throw new Exception($"The generic object type '{genericObjectTypeInfo}' does not provide the mandatory method implementation with the signature 'public static void Deserialize(XElement serializedObject, out object _object)'!");
                }
                genericObjectTypeInfo.DeserializeDelegate = (GenericObjectTypeInfo.Deserialize)deserializeMethodInfo.CreateDelegate(typeof(GenericObjectTypeInfo.Deserialize));

                IEnumerable<Type> utilizedTypes = type.GetGenericArguments().Distinct();

                genericObjectTypeInfo.PrimitiveTypeDependencies = new HashSet<PrimitiveTypeInfo>();
                genericObjectTypeInfo.NonGenericObjectTypeDependencies = new HashSet<NonGenericObjectTypeInfo>();
                genericObjectTypeInfo.GenericObjectTypeDependencies = new HashSet<GenericObjectTypeInfo>();

                foreach (Type utilizedType in utilizedTypes)
                {
                    AnalyzeType(utilizedType);

                    if (registeredUnserializableTypes.Contains(utilizedType))
                    {
                        throw new Exception($"Generic object type '{type.FullName}' utilizes unserializable type '{utilizedType.FullName}'! A generic object type cannot utilize an unserializable type!");
                    }
                    else if (registeredPrimitiveTypeInfos.ContainsKey(utilizedType))
                    {
                        genericObjectTypeInfo.PrimitiveTypeDependencies.Add(registeredPrimitiveTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (newlyRegisteredNonGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        genericObjectTypeInfo.NonGenericObjectTypeDependencies.Add(newlyRegisteredNonGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (registeredNonGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        genericObjectTypeInfo.NonGenericObjectTypeDependencies.Add(registeredNonGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (newlyRegisteredGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        genericObjectTypeInfo.GenericObjectTypeDependencies.Add(newlyRegisteredGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (registeredGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        genericObjectTypeInfo.GenericObjectTypeDependencies.Add(registeredGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (newlyRegisteredFileTypeInfos.ContainsKey(utilizedType) || registeredFileTypeInfos.ContainsKey(utilizedType))
                    {
                        throw new Exception($"Generic object type '{type.FullName}' utilizes file type '{utilizedType.FullName}'! A generic object type cannot utilize a file type!");
                    }
                    else if (newlyRegisteredFolderTypeInfos.ContainsKey(utilizedType) || registeredFolderTypeInfos.ContainsKey(utilizedType))
                    {
                        throw new Exception($"Generic object type '{type.FullName}' utilizes folder type '{utilizedType.FullName}'! A generic object type cannot utilize a folder type!");
                    }
                }

                return genericObjectTypeInfo;
            }
            else if (serializableFileAttribute != null)
            {
                FileTypeInfo fileTypeInfo = new FileTypeInfo(type);
                newlyRegisteredFileTypeInfos.Add(type, fileTypeInfo);

                fileTypeInfo.Properties = type.GetProperties(BindingFlags.Public | BindingFlags.Instance).Where((property) => property.GetGetMethod() != null && property.GetSetMethod() != null).ToArray();
                fileTypeInfo.Fields = type.GetFields(BindingFlags.Public | BindingFlags.Instance);

                fileTypeInfo.OverrideSerialization = serializableFileAttribute.OverrideSerialization;
                fileTypeInfo.OverrideDeserialization = serializableFileAttribute.OverrideDeserialization;

                if (fileTypeInfo.OverrideSerialization)
                {
                    MethodInfo methodInfo = fileTypeInfo.Type.GetMethod("Serialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(string), typeof(string), typeof(string), typeof(object), typeof(FileInfo).MakeByRefType() }, null);

                    if (methodInfo == null)
                    {
                        throw new Exception($"The file type '{fileTypeInfo.Type}' is marked as overriding the defualt serialization behaviour, but it does not implement a method with the signature 'public static void Serialize(string fileName, string fileExtension, string parentFolderPath, object file, out FileInfo serializedFile)'!");
                    }

                    fileTypeInfo.SerializeDelegate = (FileTypeInfo.Serialize)methodInfo.CreateDelegate(typeof(FileTypeInfo.Serialize));
                }
                if (fileTypeInfo.OverrideDeserialization)
                {
                    MethodInfo methodInfo = fileTypeInfo.Type.GetMethod("Deserialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(FileInfo), typeof(object).MakeByRefType() }, null);

                    if (methodInfo == null)
                    {
                        throw new Exception($"The file type '{fileTypeInfo}' is marked as overriding the defualt deserialization behaviour, but it does not implement a method with the signature 'public static void Deserialize(FileInfo serializedFile, out object file)'!");
                    }

                    fileTypeInfo.DeserializeDelegate = (FileTypeInfo.Deserialize)methodInfo.CreateDelegate(typeof(FileTypeInfo.Deserialize));
                }

                IEnumerable<Type> propertyTypes = fileTypeInfo.Properties.Select(property => property.PropertyType);
                IEnumerable<Type> fieldTypes = fileTypeInfo.Fields.Select(field => field.FieldType);
                IEnumerable<Type> utilizedTypes = propertyTypes.Concat(fieldTypes).Distinct();

                fileTypeInfo.NonGenericObjectTypeDependencies = new HashSet<NonGenericObjectTypeInfo>();
                fileTypeInfo.GenericObjectTypeDependencies = new HashSet<GenericObjectTypeInfo>();

                foreach (Type utilizedType in utilizedTypes)
                {
                    AnalyzeType(utilizedType);

                    if (registeredUnserializableTypes.Contains(utilizedType))
                    {
                        throw new Exception($"File type '{type.FullName}' utilizes unserializable type '{utilizedType.FullName}'! A file type cannot utilize an unserializable type!");
                    }
                    else if (registeredPrimitiveTypeInfos.ContainsKey(utilizedType))
                    {
                        throw new Exception($"File type '{type.FullName}' utilizes primitive type '{utilizedType.FullName}'! A file type cannot utilize a primitive type!");
                    }
                    else if (newlyRegisteredNonGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        fileTypeInfo.NonGenericObjectTypeDependencies.Add(newlyRegisteredNonGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (registeredNonGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        fileTypeInfo.NonGenericObjectTypeDependencies.Add(registeredNonGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (newlyRegisteredGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        fileTypeInfo.GenericObjectTypeDependencies.Add(newlyRegisteredGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (registeredGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        fileTypeInfo.GenericObjectTypeDependencies.Add(registeredGenericObjectTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (newlyRegisteredFileTypeInfos.ContainsKey(utilizedType) || registeredFileTypeInfos.ContainsKey(utilizedType))
                    {
                        throw new Exception($"File type '{type.FullName}' utilizes file type '{utilizedType.FullName}'! A file type cannot utilize a file type!");
                    }
                    else if (newlyRegisteredFolderTypeInfos.ContainsKey(utilizedType) || registeredFolderTypeInfos.ContainsKey(utilizedType))
                    {
                        throw new Exception($"File type '{type.FullName}' utilizes folder type '{utilizedType.FullName}'! A file type cannot utilize a folder type!");
                    }
                }

                return fileTypeInfo;
            }
            else
            {
                FolderTypeInfo folderTypeInfo = new FolderTypeInfo(type);
                newlyRegisteredFolderTypeInfos.Add(type, folderTypeInfo);

                folderTypeInfo.Properties = type.GetProperties(BindingFlags.Public | BindingFlags.Instance).Where((property) => property.GetGetMethod() != null && property.GetSetMethod() != null).ToArray();
                folderTypeInfo.Fields = type.GetFields(BindingFlags.Public | BindingFlags.Instance);

                folderTypeInfo.OverrideSerialization = serializableFolderAttribute.OverrideSerialization;
                folderTypeInfo.OverrideDeserialization = serializableFolderAttribute.OverrideDeserialization;

                if (folderTypeInfo.OverrideSerialization)
                {
                    MethodInfo methodInfo = folderTypeInfo.Type.GetMethod("Serialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(string), typeof(string), typeof(object), typeof(DirectoryInfo).MakeByRefType() }, null);

                    if (methodInfo == null)
                    {
                        throw new Exception($"The folder type '{folderTypeInfo.Type}' is marked as overriding the defualt serialization behaviour, but it does not implement a method with the signature 'public static void Serialize(string folderName, string parentFolderPath, object folder, out DirectoryInfo serializedFolder)'!");
                    }

                    folderTypeInfo.SerializeDelegate = (FolderTypeInfo.Serialize)methodInfo.CreateDelegate(typeof(FolderTypeInfo.Serialize));
                }
                if (folderTypeInfo.OverrideDeserialization)
                {
                    MethodInfo methodInfo = folderTypeInfo.Type.GetMethod("Deserialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(DirectoryInfo), typeof(object).MakeByRefType() }, null);

                    if (methodInfo == null)
                    {
                        throw new Exception($"The folder type '{folderTypeInfo}' is marked as overriding the defualt deserialization behaviour, but it does not implement a method with the signature 'public static void Deserialize(DirectoryInfo serializedFolder, out object folder)'!");
                    }

                    folderTypeInfo.DeserializeDelegate = (FolderTypeInfo.Deserialize)methodInfo.CreateDelegate(typeof(FolderTypeInfo.Deserialize));
                }

                IEnumerable<Type> propertyTypes = folderTypeInfo.Properties.Select(property => property.PropertyType);
                IEnumerable<Type> fieldTypes = folderTypeInfo.Fields.Select(field => field.FieldType);
                IEnumerable<Type> utilizedTypes = propertyTypes.Concat(fieldTypes).Distinct();

                folderTypeInfo.FileTypeDependencies = new HashSet<FileTypeInfo>();
                folderTypeInfo.FolderTypeDependencies = new HashSet<FolderTypeInfo>();

                foreach (Type utilizedType in utilizedTypes)
                {
                    AnalyzeType(utilizedType);

                    if (registeredUnserializableTypes.Contains(utilizedType))
                    {
                        throw new Exception($"Folder type '{type.FullName}' utilizes unserializable type '{utilizedType.FullName}'! A folder type cannot utilize an unserializable type!");
                    }
                    else if (registeredPrimitiveTypeInfos.ContainsKey(utilizedType))
                    {
                        throw new Exception($"Folder type '{type.FullName}' utilizes primitive type '{utilizedType.FullName}'! A folder type cannot utilize a primitive type!");
                    }
                    else if (newlyRegisteredNonGenericObjectTypeInfos.ContainsKey(utilizedType) || registeredNonGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        throw new Exception($"Folder type '{type.FullName}' utilizes non-generic object type '{utilizedType.FullName}'! A folder type cannot utilize a non-generic object type!");
                    }
                    else if (newlyRegisteredGenericObjectTypeInfos.ContainsKey(utilizedType) || registeredGenericObjectTypeInfos.ContainsKey(utilizedType))
                    {
                        throw new Exception($"Folder type '{type.FullName}' utilizes generic object type '{utilizedType.FullName}'! A folder type cannot utilize a generic object type!");
                    }
                    else if (newlyRegisteredFileTypeInfos.ContainsKey(utilizedType))
                    {
                        folderTypeInfo.FileTypeDependencies.Add(newlyRegisteredFileTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (registeredFileTypeInfos.ContainsKey(utilizedType))
                    {
                        folderTypeInfo.FileTypeDependencies.Add(registeredFileTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (newlyRegisteredFolderTypeInfos.ContainsKey(utilizedType))
                    {
                        folderTypeInfo.FolderTypeDependencies.Add(newlyRegisteredFolderTypeInfos[utilizedType]);
                        continue;
                    }
                    else if (registeredFolderTypeInfos.ContainsKey(utilizedType))
                    {
                        folderTypeInfo.FolderTypeDependencies.Add(registeredFolderTypeInfos[utilizedType]);
                        continue;
                    }
                }

                return folderTypeInfo;
            }
        }

        /// <summary>
        /// Topologically sorts ObjectTypeInfo instances based on their dependencies. This method ensures that the returned order respects dependencies between the ObjectTypeInfo instances.
        /// </summary>
        /// <param name="objectTypeInfos">The ObjectTypeInfo instances to be sorted.</param>
        /// <returns>An array of HashSet<ObjectTypeInfo> where each set represents a stage of dependencies, with earlier stages having no dependencies on later stages.</returns>
        private HashSet<ObjectTypeInfo>[] TopologicallySortObjectTypeInfos(IEnumerable<ObjectTypeInfo> objectTypeInfos)
        {
            // The resulting sorted sets of ObjectTypeInfo instances, where each set can be processed without depending on later sets.
            List<HashSet<ObjectTypeInfo>> objectTypeInfoSets = new List<HashSet<ObjectTypeInfo>>();

            // Stores the remaining dependencies for each ObjectTypeInfo instance.
            Dictionary<ObjectTypeInfo, HashSet<ObjectTypeInfo>> remainingDependencies = new Dictionary<ObjectTypeInfo, HashSet<ObjectTypeInfo>>();

            // Combining all ObjectTypeInfo instances (both new and already registered) into one set.
            HashSet<ObjectTypeInfo> combinedObjectTypeInfos = new HashSet<ObjectTypeInfo>(
                objectTypeInfos.Union(registeredNonGenericObjectTypeInfos.Values.Cast<ObjectTypeInfo>())
                               .Union(registeredGenericObjectTypeInfos.Values.Cast<ObjectTypeInfo>())
            );

            // Initialize the remaining dependencies for each ObjectTypeInfo instance.
            foreach (ObjectTypeInfo objectTypeInfo in combinedObjectTypeInfos)
            {
                HashSet<ObjectTypeInfo> dependencies = new HashSet<ObjectTypeInfo>(
                    objectTypeInfo.NonGenericObjectTypeDependencies.Cast<ObjectTypeInfo>()
                    .Union(objectTypeInfo.GenericObjectTypeDependencies)
                );
                remainingDependencies[objectTypeInfo] = dependencies;
            }

            // Set to keep track of visited ObjectTypeInfo instances to avoid redundant checks.
            HashSet<ObjectTypeInfo> visited = new HashSet<ObjectTypeInfo>();

            // Recursive function to detect circular dependencies.
            void DetectCircularDependency(ObjectTypeInfo current, HashSet<ObjectTypeInfo> path)
            {
                // If the current ObjectTypeInfo is already in the path, a circular dependency is detected.
                if (path.Contains(current))
                {
                    path.Add(current);
                    throw new Exception($"Circular dependency detected: {string.Join(" -> ", path.Select(oti => oti.Type.FullName))}");
                }

                // If the current ObjectTypeInfo has been visited, we can skip the checks.
                if (visited.Contains(current))
                {
                    return;
                }

                // Add the current ObjectTypeInfo to the path and check its dependencies.
                path.Add(current);
                foreach (ObjectTypeInfo next in remainingDependencies[current])
                {
                    DetectCircularDependency(next, path);
                }

                // Mark the current ObjectTypeInfo as visited and remove it from the current path.
                visited.Add(current);
                path.Remove(current);
            }

            // Check each ObjectTypeInfo instance for circular dependencies.
            foreach (ObjectTypeInfo objectTypeInfo in combinedObjectTypeInfos)
            {
                if (!visited.Contains(objectTypeInfo))
                {
                    DetectCircularDependency(objectTypeInfo, new HashSet<ObjectTypeInfo>());
                }
            }

            // Initialize a queue with ObjectTypeInfo instances that have no dependencies.
            Queue<ObjectTypeInfo> objectTypeInfosWithNoDependencies = new Queue<ObjectTypeInfo>(
                combinedObjectTypeInfos.Where(oti => !remainingDependencies[oti].Any())
            );

            // While there are ObjectTypeInfo instances with no dependencies, process them.
            while (objectTypeInfosWithNoDependencies.Count > 0)
            {
                HashSet<ObjectTypeInfo> currentBatch = new HashSet<ObjectTypeInfo>();

                // Process each ObjectTypeInfo instance with no dependencies.
                while (objectTypeInfosWithNoDependencies.Count > 0)
                {
                    ObjectTypeInfo currentTypeInfo = objectTypeInfosWithNoDependencies.Dequeue();
                    currentBatch.Add(currentTypeInfo);

                    // Check other ObjectTypeInfo instances that depend on the current one.
                    foreach (ObjectTypeInfo dependentObjectTypeInfo in remainingDependencies.Keys.ToList())
                    {
                        // If the current ObjectTypeInfo is a dependency of another, remove it from that ObjectTypeInfo's list of dependencies.
                        if (remainingDependencies[dependentObjectTypeInfo].Remove(currentTypeInfo) && !remainingDependencies[dependentObjectTypeInfo].Any())
                        {
                            objectTypeInfosWithNoDependencies.Enqueue(dependentObjectTypeInfo);
                        }
                    }

                    // Remove the current ObjectTypeInfo from the remaining dependencies.
                    remainingDependencies.Remove(currentTypeInfo);
                }

                // Add the current batch of ObjectTypeInfo instances to the result list.
                objectTypeInfoSets.Add(currentBatch);
            }

            // If there are any remaining dependencies, it means there's a circular dependency that wasn't identified earlier.
            if (remainingDependencies.Count != 0)
            {
                throw new Exception($"Unidentifiable circular dependency detected! This is VERY problematic and should never happen!");
            }

            return objectTypeInfoSets.ToArray();
        }

        /// <summary>
        /// Topologically sorts FolderTypeInfo instances based on their dependencies. This method ensures that the returned order respects dependencies between the FolderTypeInfo instances.
        /// </summary>
        /// <param name="folderTypeInfos">The FolderTypeInfo instances to be sorted.</param>
        /// <returns>An array of HashSet<FolderTypeInfo> where each set represents a stage of dependencies, with earlier stages having no dependencies on later stages.</returns>
        private HashSet<FolderTypeInfo>[] TopologicallySortFolderTypeInfos(IEnumerable<FolderTypeInfo> folderTypeInfos)
        {
            // The resulting sorted sets of FolderTypeInfo instances, where each set can be processed without depending on later sets.
            List<HashSet<FolderTypeInfo>> folderTypeInfoSets = new List<HashSet<FolderTypeInfo>>();

            // Stores the remaining dependencies for each FolderTypeInfo instance.
            Dictionary<FolderTypeInfo, HashSet<FolderTypeInfo>> remainingDependencies = new Dictionary<FolderTypeInfo, HashSet<FolderTypeInfo>>();

            // Combining all FolderTypeInfo instances (both new and already registered) into one set.
            HashSet<FolderTypeInfo> combinedFolderTypeInfos = new HashSet<FolderTypeInfo>(registeredFolderTypeInfos.Values);

            // Initialize the remaining dependencies for each FolderTypeInfo instance.
            foreach (FolderTypeInfo folderTypeInfo in combinedFolderTypeInfos)
            {
                HashSet<FolderTypeInfo> dependencies = new HashSet<FolderTypeInfo>(folderTypeInfo.FolderTypeDependencies);
                remainingDependencies[folderTypeInfo] = dependencies;
            }

            // Set to keep track of visited FolderTypeInfo instances to avoid redundant checks.
            HashSet<FolderTypeInfo> visited = new HashSet<FolderTypeInfo>();

            // Recursive function to detect circular dependencies.
            void DetectCircularDependency(FolderTypeInfo current, HashSet<FolderTypeInfo> path)
            {
                // If the current FolderTypeInfo is already in the path, a circular dependency is detected.
                if (path.Contains(current))
                {
                    path.Add(current);
                    throw new Exception($"Circular dependency detected: {string.Join(" -> ", path.Select(oti => oti.Type.FullName))}");
                }

                // If the current FolderTypeInfo has been visited, we can skip the checks.
                if (visited.Contains(current))
                {
                    return;
                }

                // Add the current FolderTypeInfo to the path and check its dependencies.
                path.Add(current);
                foreach (FolderTypeInfo next in remainingDependencies[current])
                {
                    DetectCircularDependency(next, path);
                }

                // Mark the current FolderTypeInfo as visited and remove it from the current path.
                visited.Add(current);
                path.Remove(current);
            }

            // Check each FolderTypeInfo instance for circular dependencies.
            foreach (FolderTypeInfo folderTypeInfo in combinedFolderTypeInfos)
            {
                if (!visited.Contains(folderTypeInfo))
                {
                    DetectCircularDependency(folderTypeInfo, new HashSet<FolderTypeInfo>());
                }
            }

            // Initialize a queue with FolderTypeInfo instances that have no dependencies.
            Queue<FolderTypeInfo> folderTypeInfosWithNoDependencies = new Queue<FolderTypeInfo>(
                combinedFolderTypeInfos.Where(fti => !remainingDependencies[fti].Any())
            );

            // While there are FolderTypeInfo instances with no dependencies, process them.
            while (folderTypeInfosWithNoDependencies.Count > 0)
            {
                HashSet<FolderTypeInfo> currentBatch = new HashSet<FolderTypeInfo>();

                // Process each FolderTypeInfo instance with no dependencies.
                while (folderTypeInfosWithNoDependencies.Count > 0)
                {
                    FolderTypeInfo currentTypeInfo = folderTypeInfosWithNoDependencies.Dequeue();
                    currentBatch.Add(currentTypeInfo);

                    // Check other FolderTypeInfo instances that depend on the current one.
                    foreach (FolderTypeInfo dependentFolderTypeInfo in remainingDependencies.Keys.ToList())
                    {
                        // If the current FolderTypeInfo is a dependency of another, remove it from that FolderTypeInfo's list of dependencies.
                        if (remainingDependencies[dependentFolderTypeInfo].Remove(currentTypeInfo) && !remainingDependencies[dependentFolderTypeInfo].Any())
                        {
                            folderTypeInfosWithNoDependencies.Enqueue(dependentFolderTypeInfo);
                        }
                    }

                    // Remove the current FolderTypeInfo from the remaining dependencies.
                    remainingDependencies.Remove(currentTypeInfo);
                }

                // Add the current batch of FolderTypeInfo instances to the result list.
                folderTypeInfoSets.Add(currentBatch);
            }

            // If there are any remaining dependencies, it means there's a circular dependency that wasn't identified earlier.
            if (remainingDependencies.Count != 0)
            {
                throw new Exception($"Unidentifiable circular dependency detected! This is VERY problematic and should never happen!");
            }

            return folderTypeInfoSets.ToArray();
        }

        private void ComposeNonGenericObjectTypeDelegates(NonGenericObjectTypeInfo nonGenericObjectTypeInfo)
        {
            if (nonGenericObjectTypeInfo.OverrideSerialization && nonGenericObjectTypeInfo.OverrideDeserialization)
            {
                return;
            }

            Dictionary<Type, PrimitiveTypeInfo> primitiveTypeDependencies = new Dictionary<Type, PrimitiveTypeInfo>();
            Dictionary<Type, NonGenericObjectTypeInfo> nonGenericObjectTypeDependencies = new Dictionary<Type, NonGenericObjectTypeInfo>();
            Dictionary<Type, GenericObjectTypeInfo> genericObjectTypeDependencies = new Dictionary<Type, GenericObjectTypeInfo>();
            
            foreach (PrimitiveTypeInfo primitiveTypeDependency in nonGenericObjectTypeInfo.PrimitiveTypeDependencies)
            {
                primitiveTypeDependencies.Add(primitiveTypeDependency.Type, primitiveTypeDependency);
            }
            foreach (NonGenericObjectTypeInfo nonGenericObjectTypeDependency in nonGenericObjectTypeInfo.NonGenericObjectTypeDependencies)
            {
                nonGenericObjectTypeDependencies.Add(nonGenericObjectTypeDependency.Type, nonGenericObjectTypeDependency);
            }
            foreach (GenericObjectTypeInfo genericObjectTypeDependency in nonGenericObjectTypeInfo.GenericObjectTypeDependencies)
            {
                genericObjectTypeDependencies.Add(genericObjectTypeDependency.Type, genericObjectTypeDependency);
            }

            Dictionary<PrimitiveTypeInfo, HashSet<PropertyInfo>> primitivePropertySets = new Dictionary<PrimitiveTypeInfo, HashSet<PropertyInfo>>();
            Dictionary<NonGenericObjectTypeInfo, HashSet<PropertyInfo>> nonGenericObjectPropertySets = new Dictionary<NonGenericObjectTypeInfo, HashSet<PropertyInfo>>();
            Dictionary<GenericObjectTypeInfo, HashSet<PropertyInfo>> genericObjectPropertySets = new Dictionary<GenericObjectTypeInfo, HashSet<PropertyInfo>>();

            foreach (PropertyInfo property in nonGenericObjectTypeInfo.Properties)
            {
                Type propertyType = property.PropertyType;
                primitiveTypeDependencies.TryGetValue(propertyType, out PrimitiveTypeInfo propertyPrimitiveTypeInfo);
                nonGenericObjectTypeDependencies.TryGetValue(propertyType, out NonGenericObjectTypeInfo propertyNonGenericObjectTypeInfo);
                genericObjectTypeDependencies.TryGetValue(propertyType, out GenericObjectTypeInfo propertyGenericObjectTypeInfo);

                if (propertyPrimitiveTypeInfo != null)
                {
                    if (!primitivePropertySets.ContainsKey(propertyPrimitiveTypeInfo))
                    {
                        primitivePropertySets[propertyPrimitiveTypeInfo] = new HashSet<PropertyInfo>();
                    }
                    primitivePropertySets[propertyPrimitiveTypeInfo].Add(property);
                }
                else if (propertyNonGenericObjectTypeInfo != null)
                {
                    if (!nonGenericObjectPropertySets.ContainsKey(propertyNonGenericObjectTypeInfo))
                    {
                        nonGenericObjectPropertySets[propertyNonGenericObjectTypeInfo] = new HashSet<PropertyInfo>();
                    }
                    nonGenericObjectPropertySets[propertyNonGenericObjectTypeInfo].Add(property);
                }
                else if (propertyGenericObjectTypeInfo != null)
                {
                    if (!genericObjectPropertySets.ContainsKey(propertyGenericObjectTypeInfo))
                    {
                        genericObjectPropertySets[propertyGenericObjectTypeInfo] = new HashSet<PropertyInfo>();
                    }
                    genericObjectPropertySets[propertyGenericObjectTypeInfo].Add(property);
                }
            }

            Dictionary<PrimitiveTypeInfo, HashSet<FieldInfo>> primitiveFieldSets = new Dictionary<PrimitiveTypeInfo, HashSet<FieldInfo>>();
            Dictionary<NonGenericObjectTypeInfo, HashSet<FieldInfo>> nonGenericObjectFieldSets = new Dictionary<NonGenericObjectTypeInfo, HashSet<FieldInfo>>();
            Dictionary<GenericObjectTypeInfo, HashSet<FieldInfo>> genericObjectFieldSets = new Dictionary<GenericObjectTypeInfo, HashSet<FieldInfo>>();

            foreach (FieldInfo field in nonGenericObjectTypeInfo.Fields)
            {
                Type fieldType = field.FieldType;
                primitiveTypeDependencies.TryGetValue(fieldType, out PrimitiveTypeInfo fieldPrimitiveTypeInfo);
                nonGenericObjectTypeDependencies.TryGetValue(fieldType, out NonGenericObjectTypeInfo fieldNonGenericObjectTypeInfo);
                genericObjectTypeDependencies.TryGetValue(fieldType, out GenericObjectTypeInfo fieldGenericObjectTypeInfo);

                if (fieldPrimitiveTypeInfo != null)
                {
                    if (!primitiveFieldSets.ContainsKey(fieldPrimitiveTypeInfo))
                    {
                        primitiveFieldSets[fieldPrimitiveTypeInfo] = new HashSet<FieldInfo>();
                    }
                    primitiveFieldSets[fieldPrimitiveTypeInfo].Add(field);
                }
                else if (fieldNonGenericObjectTypeInfo != null)
                {
                    if (!nonGenericObjectFieldSets.ContainsKey(fieldNonGenericObjectTypeInfo))
                    {
                        nonGenericObjectFieldSets[fieldNonGenericObjectTypeInfo] = new HashSet<FieldInfo>();
                    }
                    nonGenericObjectFieldSets[fieldNonGenericObjectTypeInfo].Add(field);
                }
                else if (fieldGenericObjectTypeInfo != null)
                {
                    if (!genericObjectFieldSets.ContainsKey(fieldGenericObjectTypeInfo))
                    {
                        genericObjectFieldSets[fieldGenericObjectTypeInfo] = new HashSet<FieldInfo>();
                    }
                    genericObjectFieldSets[fieldGenericObjectTypeInfo].Add(field);
                }
            }

            if (!nonGenericObjectTypeInfo.OverrideSerialization)
            {
                nonGenericObjectTypeInfo.SerializeDelegate = (string objectName, object _object, out XElement serializedObject) =>
                {
                    serializedObject = new XElement(objectName);

                    foreach (KeyValuePair<PrimitiveTypeInfo, HashSet<PropertyInfo>> primitivePropertySetKeyValuePair in primitivePropertySets)
                    {
                        foreach (PropertyInfo property in primitivePropertySetKeyValuePair.Value)
                        {
                            primitivePropertySetKeyValuePair.Key.SerializeDelegate.Invoke(property.Name, property.GetValue(_object), out XAttribute serializedPrimitive);
                            serializedObject.Add(serializedPrimitive);
                        }
                    }

                    foreach (KeyValuePair<NonGenericObjectTypeInfo, HashSet<PropertyInfo>> nonGenericObjectPropertySetKeyValuePair in nonGenericObjectPropertySets)
                    {
                        foreach (PropertyInfo property in nonGenericObjectPropertySetKeyValuePair.Value)
                        {
                            nonGenericObjectPropertySetKeyValuePair.Key.SerializeDelegate.Invoke(property.Name, property.GetValue(_object), out XElement serializedNonGenericObject);
                            serializedObject.Add(serializedNonGenericObject);
                        }
                    }

                    foreach (KeyValuePair<GenericObjectTypeInfo, HashSet<PropertyInfo>> genericObjectPropertySetKeyValuePair in genericObjectPropertySets)
                    {
                        foreach (PropertyInfo property in genericObjectPropertySetKeyValuePair.Value)
                        {
                            genericObjectPropertySetKeyValuePair.Key.SerializeDelegate.Invoke(property.Name, property.GetValue(_object), out XElement serializedGenericObject);
                            serializedObject.Add(serializedGenericObject);
                        }
                    }

                    foreach (KeyValuePair<PrimitiveTypeInfo, HashSet<FieldInfo>> primitiveFieldSetKeyValuePair in primitiveFieldSets)
                    {
                        foreach (FieldInfo field in primitiveFieldSetKeyValuePair.Value)
                        {
                            primitiveFieldSetKeyValuePair.Key.SerializeDelegate.Invoke(field.Name, field.GetValue(_object), out XAttribute serializedPrimitive);
                            serializedObject.Add(serializedPrimitive);
                        }
                    }

                    foreach (KeyValuePair<NonGenericObjectTypeInfo, HashSet<FieldInfo>> nonGenericObjectFieldSetKeyValuePair in nonGenericObjectFieldSets)
                    {
                        foreach (FieldInfo field in nonGenericObjectFieldSetKeyValuePair.Value)
                        {
                            nonGenericObjectFieldSetKeyValuePair.Key.SerializeDelegate.Invoke(field.Name, field.GetValue(_object), out XElement serializedNonGenericObject);
                            serializedObject.Add(serializedNonGenericObject);
                        }
                    }

                    foreach (KeyValuePair<GenericObjectTypeInfo, HashSet<FieldInfo>> genericObjectFieldSetKeyValuePair in genericObjectFieldSets)
                    {
                        foreach (FieldInfo field in genericObjectFieldSetKeyValuePair.Value)
                        {
                            genericObjectFieldSetKeyValuePair.Key.SerializeDelegate.Invoke(field.Name, field.GetValue(_object), out XElement serializedGenericObject);
                            serializedObject.Add(serializedGenericObject);
                        }
                    }
                }; 
            }

            if (!nonGenericObjectTypeInfo.OverrideDeserialization)
            {
                nonGenericObjectTypeInfo.DeserializeDelegate = (XElement serializedObject, out object _object) =>
                {
                    _object = Activator.CreateInstance(nonGenericObjectTypeInfo.Type);

                    Dictionary<string, XAttribute> serializedPrimitives = new Dictionary<string, XAttribute>();
                    Dictionary<string, XElement> serializedObjects = new Dictionary<string, XElement>();

                    foreach (XAttribute serializedPrimitive in serializedObject.Attributes())
                    {
                        serializedPrimitives.Add(serializedPrimitive.Name.ToString(), serializedPrimitive);
                    }
                    foreach (XElement serializedNonGenericObject in serializedObject.Elements())
                    {
                        serializedObjects.Add(serializedNonGenericObject.Name.ToString(), serializedNonGenericObject);
                    }

                    foreach (KeyValuePair<PrimitiveTypeInfo, HashSet<PropertyInfo>> primitivePropertySetKeyValuePair in primitivePropertySets)
                    {
                        foreach (PropertyInfo property in primitivePropertySetKeyValuePair.Value)
                        {
                            if (!serializedPrimitives.TryGetValue(property.Name, out XAttribute serializedPrimitive))
                            {
                                throw new Exception($"Serialized non-generic object does not contain a serialized primitive with the name {property.Name}!");
                            }
                            primitivePropertySetKeyValuePair.Key.DeserializeDelegate.Invoke(serializedPrimitive, out object deserializedPrimitive);
                            property.SetValue(_object, deserializedPrimitive);
                        }
                    }

                    foreach (KeyValuePair<NonGenericObjectTypeInfo, HashSet<PropertyInfo>> nonGenericObjectPropertySetKeyValuePair in nonGenericObjectPropertySets)
                    {
                        foreach (PropertyInfo property in nonGenericObjectPropertySetKeyValuePair.Value)
                        {
                            if (!serializedObjects.TryGetValue(property.Name, out XElement serializedNonGenericObject))
                            {
                                throw new Exception($"Serialized non-generic object does not contain a serialized non-generic object with the name {property.Name}!");
                            }
                            nonGenericObjectPropertySetKeyValuePair.Key.DeserializeDelegate.Invoke(serializedNonGenericObject, out object deserializedNonGenericObject);
                            property.SetValue(_object, deserializedNonGenericObject);
                        }
                    }

                    foreach (KeyValuePair<GenericObjectTypeInfo, HashSet<PropertyInfo>> genericObjectPropertySetKeyValuePair in genericObjectPropertySets)
                    {
                        foreach (PropertyInfo property in genericObjectPropertySetKeyValuePair.Value)
                        {
                            if (!serializedObjects.TryGetValue(property.Name, out XElement serializedGenericObject))
                            {
                                throw new Exception($"Serialized non-generic object does not contain a serialized generic object with the name {property.Name}!");
                            }
                            genericObjectPropertySetKeyValuePair.Key.DeserializeDelegate.Invoke(serializedGenericObject, out object deserializedGenericObject);
                            property.SetValue(_object, deserializedGenericObject);
                        }
                    }

                    foreach (KeyValuePair<PrimitiveTypeInfo, HashSet<FieldInfo>> primitiveFieldSetKeyValuePair in primitiveFieldSets)
                    {
                        foreach (FieldInfo field in primitiveFieldSetKeyValuePair.Value)
                        {
                            if (!serializedPrimitives.TryGetValue(field.Name, out XAttribute serializedPrimitive))
                            {
                                throw new Exception($"Serialized non-generic object does not contain a serialized primitive with the name {field.Name}!");
                            }
                            primitiveFieldSetKeyValuePair.Key.DeserializeDelegate.Invoke(serializedPrimitive, out object deserializedPrimitive);
                            field.SetValue(_object, deserializedPrimitive);
                        }
                    }

                    foreach (KeyValuePair<NonGenericObjectTypeInfo, HashSet<FieldInfo>> nonGenericObjectFieldSetKeyValuePair in nonGenericObjectFieldSets)
                    {
                        foreach (FieldInfo field in nonGenericObjectFieldSetKeyValuePair.Value)
                        {
                            if (!serializedObjects.TryGetValue(field.Name, out XElement serializedNonGenericObject))
                            {
                                throw new Exception($"Serialized non-generic object does not contain a serialized non-generic object with the name {field.Name}!");
                            }
                            nonGenericObjectFieldSetKeyValuePair.Key.DeserializeDelegate.Invoke(serializedNonGenericObject, out object deserializedNonGenericObject);
                            field.SetValue(_object, deserializedNonGenericObject);
                        }
                    }

                    foreach (KeyValuePair<GenericObjectTypeInfo, HashSet<FieldInfo>> genericObjectFieldSetKeyValuePair in genericObjectFieldSets)
                    {
                        foreach (FieldInfo field in genericObjectFieldSetKeyValuePair.Value)
                        {
                            if (!serializedObjects.TryGetValue(field.Name, out XElement serializedGenericObject))
                            {
                                throw new Exception($"Serialized non-generic object does not contain a serialized generic object with the name {field.Name}!");
                            }
                            genericObjectFieldSetKeyValuePair.Key.DeserializeDelegate.Invoke(serializedGenericObject, out object deserializedGenericObject);
                            field.SetValue(_object, deserializedGenericObject);
                        }
                    }
                }; 
            }
        }

        private void ComposeFileTypeDelegates(FileTypeInfo fileTypeInfo)
        {

        }

        private void ComposeFolderTypeDelegates(FolderTypeInfo folderTypeInfo)
        {

        }
        #endregion
    }
}
