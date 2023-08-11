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
    using LooCast.System.Exceptions;

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
        private Dictionary<Type, string> invalidations;
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
            invalidations = new Dictionary<Type, string>();

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
                int cachedTypeCount = registeredUnserializableTypes.Count + registeredPrimitiveTypeInfos.Count + registeredNonGenericObjectTypeInfos.Count + registeredGenericObjectTypeInfos.Count + registeredFileTypeInfos.Count + registeredFolderTypeInfos.Count;
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

        #region Type Management
        /// <summary>
        /// For performance reasons it is highly recommended to never register only a single type at once!
        /// </summary>
        public void RegisterTypes(IEnumerable<Type> types)
        {
            HashSet<Type> nonGenericObjectTypes = new HashSet<Type>();
            HashSet<Type> genericObjectTypes = new HashSet<Type>();
            HashSet<Type> fileTypes = new HashSet<Type>();
            HashSet<Type> folderTypes = new HashSet<Type>();
            
            foreach (Type type in types)
            {
                if (registeredUnserializableTypes.Contains(type))
                {
                    throw new InvalidOperationException($"Type '{type.FullName}' is already registered as unserializable!");
                }
                if (registeredPrimitiveTypeInfos.ContainsKey(type))
                {
                    throw new InvalidOperationException($"Type '{type.FullName}' is already registered as primitive!");
                }
                
                if (type.IsGenericTypeDefinition)
                {
                    registeredUnserializableTypes.Add(type);
                    continue;
                }
                if (type.IsAbstract)
                {
                    registeredUnserializableTypes.Add(type);
                    continue;
                }
                if (!type.IsPublic && !type.IsNestedPublic)
                {
                    registeredUnserializableTypes.Add(type);
                    continue;
                }
                if (!type.IsClass && !type.IsValueType && !type.IsEnum)
                {
                    registeredUnserializableTypes.Add(type);
                    continue;
                }

                SerializableNonGenericObjectAttribute serializableNonGenericObjectAttribute = type.GetCustomAttribute<SerializableNonGenericObjectAttribute>(false);
                SerializableGenericObjectAttribute serializableGenericObjectAttribute = type.GetCustomAttribute<SerializableGenericObjectAttribute>(false);
                SerializableFileAttribute serializableFileAttribute = type.GetCustomAttribute<SerializableFileAttribute>(false);
                SerializableFolderAttribute serializableFolderAttribute = type.GetCustomAttribute<SerializableFolderAttribute>(false);

                if (serializableNonGenericObjectAttribute == null && serializableGenericObjectAttribute == null && serializableFileAttribute == null && serializableFolderAttribute == null)
                {
                    throw new Exception($"Type '{type.FullName}' is not marked as serializable!");
                }
                if (!(serializableNonGenericObjectAttribute != null ^ serializableGenericObjectAttribute != null ^ serializableFileAttribute != null ^ serializableFolderAttribute != null))
                {
                    throw new Exception($"Type '{type.FullName}' is marked as more than one serializable type!");
                }

                if (serializableNonGenericObjectAttribute != null)
                {
                    nonGenericObjectTypes.Add(type);
                    continue;
                }
                else if (serializableGenericObjectAttribute != null)
                {
                    genericObjectTypes.Add(type);
                    continue;
                }
                else if (serializableFileAttribute != null)
                {
                    fileTypes.Add(type);
                    continue;
                }
                else
                {
                    folderTypes.Add(type);
                    continue;
                }
            }
            
            HashSet<NonGenericObjectTypeInfo> nonGenericObjectTypeInfos = new HashSet<NonGenericObjectTypeInfo>();
            foreach (Type nonGenericObjectType in nonGenericObjectTypes)
            {
                NonGenericObjectTypeInfo nonGenericObjectTypeInfo = new NonGenericObjectTypeInfo(nonGenericObjectType);
            }

            HashSet<GenericObjectTypeInfo> genericObjectTypeInfos = new HashSet<GenericObjectTypeInfo>();
            foreach (Type genericObjectType in genericObjectTypes)
            {
                GenericObjectTypeInfo genericObjectTypeInfo = new GenericObjectTypeInfo(genericObjectType);
            }

            HashSet<FileTypeInfo> fileTypeInfos = new HashSet<FileTypeInfo>();
            foreach (Type fileType in fileTypes)
            {
                FileTypeInfo fileTypeInfo = new FileTypeInfo(fileType);
            }

            HashSet<FolderTypeInfo> folderTypeInfos = new HashSet<FolderTypeInfo>();
            foreach (Type folderType in folderTypes)
            {
                FolderTypeInfo folderTypeInfo = new FolderTypeInfo(folderType);
            }

            PreAnalyzeNonGenericObjectTypes(nonGenericObjectTypeInfos);
            PreAnalyzeFileTypes(fileTypeInfos);
            PreAnalyzeFolderTypes(folderTypeInfos);

            AnalyzeNonGenericObjectTypes(nonGenericObjectTypeInfos);
            AnalyzeGenericObjectTypes(genericObjectTypeInfos);
            AnalyzeFileTypes(fileTypeInfos);
            AnalyzeFolderTypes(folderTypeInfos);

            PreProcessNonGenericObjectTypes(nonGenericObjectTypeInfos);
            PreProcessFileTypes(fileTypeInfos);
            PreProcessFolderTypes(folderTypeInfos);

            ProcessNonGenericObjectTypes(nonGenericObjectTypeInfos);
            ProcessGenericObjectTypes(genericObjectTypeInfos);
            ProcessFileTypes(fileTypeInfos);
            ProcessFolderTypes(folderTypeInfos);

            // Log all invalidated types somehow. The idea is that some types may be marked as serializable, but ultimately aren't, but are not important enough to throw an exception.
            // If however some dependency is not met as a consequence of the invalidity, an exception is thrown.
            // If everything goes to plan regarding the dependencies, the system then logs a Severe Warning about the invalid types,
            // stating that they will be considered unserializable by the SerializationManager and will probably cause issues down the line.
            // This is a severe warning and not a hard error, because technically this does not guarantee issues down the line, but it's very likely.
        }

        /// <summary>
        /// For performance reasons it is highly recommended to never unregister only a single type at once!
        /// </summary>
        public void UnregisterTypes(IEnumerable<Type> types)
        {
            
        }
        #endregion

        #region Type Analysis & Processing
        
        #region Non-Generic Object Type
        private void PreAnalyzeNonGenericObjectTypes(IEnumerable<NonGenericObjectTypeInfo> nonGenericObjectTypeInfos)
        {
            foreach (NonGenericObjectTypeInfo nonGenericObjectTypeInfo in nonGenericObjectTypeInfos)
            {
                MethodInfo serializeMethodInfo = nonGenericObjectTypeInfo.Type.GetMethod("Serialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(string), typeof(object), typeof(XElement).MakeByRefType() }, null);
                MethodInfo deserializeMethodInfo = nonGenericObjectTypeInfo.Type.GetMethod("Deserialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(XElement), typeof(object).MakeByRefType() }, null);

                bool fullyOverridden = serializeMethodInfo != null && deserializeMethodInfo != null;

                if (nonGenericObjectTypeInfo.Type.GetConstructor(Type.EmptyTypes) == null && !fullyOverridden)
                {
                    nonGenericObjectTypeInfo.Invalidate();
                    invalidations.Add(nonGenericObjectTypeInfo.Type, "A type marked as serializable non-generic object is required to have a parameterless constructor or implement both a 'public static void Serialize(string objectName, object serializableObject, out XElement serializedObject)' and a 'public static void Deserialize(XElement serializedObject, out object serializableObject)' method!");
                    continue;
                }
            }
        }

        private void AnalyzeNonGenericObjectTypes(IEnumerable<NonGenericObjectTypeInfo> nonGenericObjectTypeInfos)
        {
            
        }
        
        private void PreProcessNonGenericObjectTypes(IEnumerable<NonGenericObjectTypeInfo> nonGenericObjectTypeInfos)
        {

        }

        private void ProcessNonGenericObjectTypes(IEnumerable<NonGenericObjectTypeInfo> nonGenericObjectTypeInfos)
        {

        }
        #endregion

        #region Generic Object Type
        private void AnalyzeGenericObjectTypes(IEnumerable<GenericObjectTypeInfo> genericObjectTypeInfos)
        {
            foreach (GenericObjectTypeInfo genericObjectTypeInfo in genericObjectTypeInfos)
            {
                MethodInfo serializeMethodInfo = genericObjectTypeInfo.Type.GetMethod("Serialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(string), typeof(object), typeof(XElement).MakeByRefType() }, null);
                MethodInfo deserializeMethodInfo = genericObjectTypeInfo.Type.GetMethod("Deserialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(XElement), typeof(object).MakeByRefType() }, null);

                bool fullyOverridden = serializeMethodInfo != null && deserializeMethodInfo != null;

                if (!fullyOverridden)
                {
                    genericObjectTypeInfo.Invalidate();
                    invalidations.Add(genericObjectTypeInfo.Type, "A type marked as serializable generic object is required to implement both a 'public static void Serialize(string objectName, object serializableObject, out XElement serializedObject)' and a 'public static void Deserialize(XElement serializedObject, out object serializableObject)' method!");
                    continue;
                }
            }
        }

        private void ProcessGenericObjectTypes(IEnumerable<GenericObjectTypeInfo> genericObjectTypeInfos)
        {

        }
        #endregion

        #region File Type
        private void PreAnalyzeFileTypes(IEnumerable<FileTypeInfo> fileTypeInfos)
        {
            foreach (FileTypeInfo fileTypeInfo in fileTypeInfos)
            {
                MethodInfo serializeMethodInfo = fileTypeInfo.Type.GetMethod("Serialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(string), typeof(string), typeof(string), typeof(object), typeof(FileInfo).MakeByRefType() }, null);
                MethodInfo deserializeMethodInfo = fileTypeInfo.Type.GetMethod("Deserialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(FileInfo), typeof(object).MakeByRefType() }, null);

                bool fullyOverridden = serializeMethodInfo != null && deserializeMethodInfo != null;

                if (fileTypeInfo.Type.GetConstructor(Type.EmptyTypes) == null && !fullyOverridden)
                {
                    fileTypeInfo.Invalidate();
                    invalidations.Add(fileTypeInfo.Type, "A type marked as serializable file is required to implement both a 'public static void Serialize(string fileName, string fileExtension, string parentFolderPath, object serializableFile, out FileInfo serializedFile)' and a 'public static void Deserialize(FileInfo serializedFile, out object serializableFile)' method!");
                    continue;
                }
            }
        }

        private void AnalyzeFileTypes(IEnumerable<FileTypeInfo> fileTypeInfos)
        {

        }

        private void PreProcessFileTypes(IEnumerable<FileTypeInfo> fileTypeInfos)
        {

        }

        private void ProcessFileTypes(IEnumerable<FileTypeInfo> fileTypeInfos)
        {

        }
        #endregion

        #region Folder Type
        private void PreAnalyzeFolderTypes(IEnumerable<FolderTypeInfo> folderTypeInfos)
        {
            foreach (FolderTypeInfo folderTypeInfo in folderTypeInfos)
            {
                MethodInfo serializeMethodInfo = folderTypeInfo.Type.GetMethod("Serialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(string), typeof(string), typeof(object), typeof(DirectoryInfo).MakeByRefType() }, null);
                MethodInfo deserializeMethodInfo = folderTypeInfo.Type.GetMethod("Deserialize", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(DirectoryInfo), typeof(object).MakeByRefType() }, null);

                bool fullyOverridden = serializeMethodInfo != null && deserializeMethodInfo != null;

                if (folderTypeInfo.Type.GetConstructor(Type.EmptyTypes) == null && !fullyOverridden)
                {
                    folderTypeInfo.Invalidate();
                    invalidations.Add(folderTypeInfo.Type, "A type marked as serializable folder is required to implement both a 'public static void Serialize(string folderName, string parentFolderPath, object serializableFolder, out DirectoryInfo serializedFolder)' and a 'public static void Deserialize(DirectoryInfo serializedFolder, out object serializableFolder)' method!");
                    continue;
                }
            }
        }

        private void AnalyzeFolderTypes(IEnumerable<FolderTypeInfo> folderTypeInfos)
        {

        }

        private void PreProcessFolderTypes(IEnumerable<FolderTypeInfo> folderTypeInfos)
        {

        }

        private void ProcessFolderTypes(IEnumerable<FolderTypeInfo> folderTypeInfos)
        {

        }
        #endregion

        #endregion

        #endregion
    }
}
