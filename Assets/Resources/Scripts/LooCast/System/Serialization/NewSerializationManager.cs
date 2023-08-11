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

    public sealed class NewSerializationManager : ModuleManager
    {
        #region Static Properties
        public static NewSerializationManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<NewSerializationManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static NewSerializationManager instance;
        #endregion

        #region Fields
        private HashSet<Type> unserializableTypes;
        private HashSet<TypeInfo> primitiveTypeInfos;
        private HashSet<TypeInfo> objectTypes;
        private HashSet<TypeInfo> fileTypes;
        private HashSet<TypeInfo> folderTypes;
        #endregion

        #region Constructors
        public NewSerializationManager() : base()
        {
            unserializableTypes = new HashSet<Type>();
            primitiveTypeInfos = new HashSet<TypeInfo>();
            objectTypes = new HashSet<TypeInfo>();
            fileTypes = new HashSet<TypeInfo>();
            folderTypes = new HashSet<TypeInfo>();

            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedComponentManagerEntityTypeName = typeof(NewSerializationManager).AssemblyQualifiedName;
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
                componentManagerData.ManagerName = "NewSerializationManager";
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
                primitiveSerializationDelegates.Add(byteType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(byteType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(sbyteType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(sbyteType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(charType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(charType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(decimalType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(decimalType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(doubleType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(doubleType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(floatType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(floatType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(intType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(intType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(uintType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(uintType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(longType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(longType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(ulongType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(ulongType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(shortType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(shortType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(ushortType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(ushortType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(stringType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(stringType, (XAttribute serializedPrimitive, out object primitive) =>
                {
                    if (serializedPrimitive == null)
                    {
                        throw new ArgumentNullException(nameof(serializedPrimitive));
                    }

                    primitive = serializedPrimitive.Value;
                });

                Type guidType = typeof(Guid);
                primitiveSerializationDelegates.Add(guidType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(guidType, (XAttribute serializedPrimitive, out object primitive) =>
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
                primitiveSerializationDelegates.Add(bigIntType, (string primitiveName, object primitive, out XAttribute serializedPrimitive) =>
                {
                    serializedPrimitive = new XAttribute(primitiveName, primitive);
                });
                primitiveDeserializationDelegates.Add(bigIntType, (XAttribute serializedPrimitive, out object primitive) =>
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

                #region Type caching
                Stopwatch stopwatch = new Stopwatch();
                stopwatch.Start();

                IEnumerable<Assembly> allInitialAssemblyDefinitions = AppDomain.CurrentDomain.GetAssemblies();
                IEnumerable<Type> allInitialTypeDefinitions = allInitialAssemblyDefinitions.SelectMany(assembly => assembly.GetTypes());

                // TODO: Maybe somehow more clearly distinguish between the 5 phases of the type caching:
                // 1.   Determine & Cache type meta info
                // 2.   Determine & Cache type serializability
                // 3.   Determine & Cache type info
                // 4.   Determine & Cache type serialization delegates
                // 5.   Determine & Cache type serialization sub delegates

                // TODO: Maybe somehow make this whole process a bit more extensible, so you can easily load and unload any serializable type
                // TODO: Also make sure that the type caching is done in a way that it can be done in parallel, so that it can be done in a separate thread
                // TODO: Also make sure that the type caching is designed to cache multiple types with one method call, because implementing this system for a single type and then executing that multiple times would be very inefficient.
                // TODO: Also like create a TypeManager or AssemblyManager or CSharpManager or something like that, which keeps track of loaded assemblies and types, and can load/unload those in a managed way,
                // so that the NewSerializationManager is informed of Type additions/deletions and can update its caching cache accordingly

                // TODO: But before all of this, make sure that the base system works as intended, before adding new features to it

                foreach (Type initialTypeDefinition in allInitialTypeDefinitions)
                {
                    if (!initialTypeDefinition.IsGenericTypeDefinition)
                    {
                        if (!serializabilityCache.ContainsKey(initialTypeDefinition))
                        {
                            CacheSerializableTypeMetaInfo(initialTypeDefinition);
                        }
                    }
                }

                foreach (KeyValuePair<Type, SerializableTypeMetaInfo> serializableTypeMetaInfoCacheKeyValuePair in serializableTypeMetaInfoCache)
                {
                    CacheSerializability(serializableTypeMetaInfoCacheKeyValuePair.Key, serializableTypeMetaInfoCacheKeyValuePair.Value);
                }

                // TODO: 2. Figure out how to implement the overriding of this caching
                // Maybe the solution is to allow for a complete override of the serialization process, including type meta info caching, type serializability caching, type info caching, etc.
                // Don't forget to implement the required methods in the classes which override the behaviour
                foreach (Type serializablePrimitiveType in primitiveTypeInfos)
                {
                    CacheSerializablePrimitiveTypeInfo(serializablePrimitiveType, serializablePrimitiveTypeMetaInfoCache[serializablePrimitiveType]);
                }
                foreach (Type serializableObjectType in objectTypes)
                {
                    CacheSerializableObjectTypeInfo(serializableObjectType, serializableObjectTypeMetaInfoCache[serializableObjectType]);
                }
                foreach (Type serializableFileType in fileTypes)
                {
                    CacheSerializableFileTypeInfo(serializableFileType, serializableFileTypeMetaInfoCache[serializableFileType]);
                }
                foreach (Type serializableFolderType in folderTypes)
                {
                    CacheSerializableFolderTypeInfo(serializableFolderType, serializableFolderTypeMetaInfoCache[serializableFolderType]);
                }

                List<List<SerializableObjectTypeInfo>> initialSerializableObjectTypeInfoQueues = GetSortedObjectTypeInfos();
                SerializableFileTypeInfo[] initialSerializableFileTypeDefinitionInfos = serializableFileTypeInfoCache.Values.ToArray();
                List<List<SerializableFolderTypeInfo>> initialSerializableFolderTypeInfoQueues = GetSortedFolderTypeInfos();

                for (int i = 0; i < initialSerializableObjectTypeInfoQueues.Count; i++)
                {
                    List<SerializableObjectTypeInfo> initialSerializableObjectTypeQueue = initialSerializableObjectTypeInfoQueues[i];

                    for (int j = 0; j < initialSerializableObjectTypeQueue.Count; j++)
                    {
                        SerializableObjectTypeInfo initialSerializableObjectTypeInfo = initialSerializableObjectTypeQueue[j];
                        CacheObjectSerializationDelegates(initialSerializableObjectTypeInfo);
                    }
                }
                for (int i = 0; i < initialSerializableFileTypeDefinitionInfos.Length; i++)
                {
                    SerializableFileTypeInfo initialSerializableFileTypeInfo = initialSerializableFileTypeDefinitionInfos[i];
                    CacheFileSerializationDelegates(initialSerializableFileTypeInfo);
                }
                for (int i = 0; i < initialSerializableFolderTypeInfoQueues.Count; i++)
                {
                    List<SerializableFolderTypeInfo> initialSerializableFolderTypeQueue = initialSerializableFolderTypeInfoQueues[i];

                    for (int j = 0; j < initialSerializableFolderTypeQueue.Count; j++)
                    {
                        SerializableFolderTypeInfo initialSerializableFolderTypeInfo = initialSerializableFolderTypeQueue[j];
                        CacheFolderSerializationDelegates(initialSerializableFolderTypeInfo);
                    }
                }

                for (int i = 0; i < initialSerializableObjectTypeInfoQueues.Count; i++)
                {
                    List<SerializableObjectTypeInfo> initialSerializableObjectTypeQueue = initialSerializableObjectTypeInfoQueues[i];

                    for (int j = 0; j < initialSerializableObjectTypeQueue.Count; j++)
                    {
                        SerializableObjectTypeInfo initialSerializableObjectTypeInfo = initialSerializableObjectTypeQueue[j];
                        CacheObjectSerializationSubDelegates(initialSerializableObjectTypeInfo);
                    }
                }
                for (int i = 0; i < initialSerializableFileTypeDefinitionInfos.Length; i++)
                {
                    SerializableFileTypeInfo initialSerializableFileTypeInfo = initialSerializableFileTypeDefinitionInfos[i];
                    CacheFileSerializationSubDelegates(initialSerializableFileTypeInfo);
                }
                for (int i = 0; i < initialSerializableFolderTypeInfoQueues.Count; i++)
                {
                    List<SerializableFolderTypeInfo> initialSerializableFolderTypeQueue = initialSerializableFolderTypeInfoQueues[i];

                    for (int j = 0; j < initialSerializableFolderTypeQueue.Count; j++)
                    {
                        SerializableFolderTypeInfo initialSerializableFolderTypeInfo = initialSerializableFolderTypeQueue[j];
                        CacheFolderSerializationSubDelegates(initialSerializableFolderTypeInfo);
                    }
                }

                stopwatch.Stop();
                int cachedTypeCount = unserializableTypes.Count + primitiveTypeInfos.Count + objectTypes.Count + fileTypes.Count + folderTypes.Count;
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

        #region Delegate Management
        public SerializePrimitiveDelegate GetPrimitiveSerializationDelegate(Type serializablePrimitiveType)
        {
            if (serializablePrimitiveType == null)
            {
                throw new ArgumentNullException(nameof(serializablePrimitiveType));
            }

            if (!primitiveSerializationDelegates.ContainsKey(serializablePrimitiveType))
            {
                throw new InvalidOperationException($"No primitive serialization delegate registered for type '{serializablePrimitiveType}'!");
            }

            return primitiveSerializationDelegates[serializablePrimitiveType];
        }
        public DeserializePrimitiveDelegate GetPrimitiveDeserializationDelegate(Type serializablePrimitiveType)
        {
            if (serializablePrimitiveType == null)
            {
                throw new ArgumentNullException(nameof(serializablePrimitiveType));
            }

            if (!primitiveDeserializationDelegates.ContainsKey(serializablePrimitiveType))
            {
                throw new InvalidOperationException($"No primitive deserialization delegate registered for type '{serializablePrimitiveType}'!");
            }

            return primitiveDeserializationDelegates[serializablePrimitiveType];
        }

        public SerializeObjectDelegate GetObjectSerializationDelegate(Type serializableObjectType)
        {
            if (serializableObjectType == null)
            {
                throw new ArgumentNullException(nameof(serializableObjectType));
            }

            if (!objectSerializationDelegates.ContainsKey(serializableObjectType))
            {
                throw new InvalidOperationException($"No object serialization delegate registered for type '{serializableObjectType}'!");
            }

            return objectSerializationDelegates[serializableObjectType];
        }
        public DeserializeObjectDelegate GetObjectDeserializationDelegate(Type serializableObjectType)
        {
            if (serializableObjectType == null)
            {
                throw new ArgumentNullException(nameof(serializableObjectType));
            }

            if (!objectDeserializationDelegates.ContainsKey(serializableObjectType))
            {
                throw new InvalidOperationException($"No object deserialization delegate registered for type '{serializableObjectType}'!");
            }

            return objectDeserializationDelegates[serializableObjectType];
        }

        public SerializeFileDelegate GetFileSerializationDelegate(Type serializableFileType)
        {
            if (serializableFileType == null)
            {
                throw new ArgumentNullException(nameof(serializableFileType));
            }

            if (!fileSerializationDelegates.ContainsKey(serializableFileType))
            {
                throw new InvalidOperationException($"No file serialization delegate registered for type '{serializableFileType}'!");
            }

            return fileSerializationDelegates[serializableFileType];
        }
        public DeserializeFileDelegate GetFileDeserializationDelegate(Type serializableFileType)
        {
            if (serializableFileType == null)
            {
                throw new ArgumentNullException(nameof(serializableFileType));
            }

            if (!fileDeserializationDelegates.ContainsKey(serializableFileType))
            {
                throw new InvalidOperationException($"No file deserialization delegate registered for type '{serializableFileType}'!");
            }

            return fileDeserializationDelegates[serializableFileType];
        }

        public SerializeFolderDelegate GetFolderSerializationDelegate(Type serializableFolderType)
        {
            if (serializableFolderType == null)
            {
                throw new ArgumentNullException(nameof(serializableFolderType));
            }

            if (!folderSerializationDelegates.ContainsKey(serializableFolderType))
            {
                throw new InvalidOperationException($"No folder serialization delegate registered for type '{serializableFolderType}'!");
            }

            return folderSerializationDelegates[serializableFolderType];
        }
        public DeserializeFolderDelegate GetFolderDeserializationDelegate(Type serializableFolderType)
        {
            if (serializableFolderType == null)
            {
                throw new ArgumentNullException(nameof(serializableFolderType));
            }

            if (!folderDeserializationDelegates.ContainsKey(serializableFolderType))
            {
                throw new InvalidOperationException($"No folder deserialization delegate registered for type '{serializableFolderType}'!");
            }

            return folderDeserializationDelegates[serializableFolderType];
        }

        public bool IsPrimitiveTypeSerializationDelegateRegistered(Type serializablePrimitiveType)
        {
            return primitiveSerializationDelegates.ContainsKey(serializablePrimitiveType);
        }
        public bool IsPrimitiveTypeDeserializationDelegateRegistered(Type serializablePrimitiveType)
        {
            return primitiveDeserializationDelegates.ContainsKey(serializablePrimitiveType);
        }

        public bool IsObjectTypeSerializationDelegateRegistered(Type serializableObjectType)
        {
            return objectSerializationDelegates.ContainsKey(serializableObjectType);
        }
        public bool IsObjectTypeDeserializationDelegateRegistered(Type serializableObjectType)
        {
            return objectDeserializationDelegates.ContainsKey(serializableObjectType);
        }

        public bool IsFileTypeSerializationDelegateRegistered(Type serializableFileType)
        {
            return objectSerializationDelegates.ContainsKey(serializableFileType);
        }
        public bool IsFileTypeDeserializationDelegateRegistered(Type serializableFileType)
        {
            return objectDeserializationDelegates.ContainsKey(serializableFileType);
        }

        public bool IsFolderTypeSerializationDelegateRegistered(Type serializableFolderType)
        {
            return objectSerializationDelegates.ContainsKey(serializableFolderType);
        }
        public bool IsFolderTypeDeserializationDelegateRegistered(Type serializableFolderType)
        {
            return objectDeserializationDelegates.ContainsKey(serializableFolderType);
        }
        #endregion

        public void SerializeFile<SerializableFileType>(string fileName, string fileExtension, string parentFolderPath, SerializableFileType serializableFile, out FileInfo serializedFile)
        {
            if (serializableFile == null)
            {
                throw new ArgumentNullException(nameof(serializableFile));
            }

            Type serializableFileType = typeof(SerializableFileType);

            if (!IsFileTypeSerializationDelegateRegistered(serializableFileType))
            {
                throw new InvalidOperationException($"No file serialization delegate registered for type '{serializableFileType}'!");
            }

            fileSerializationDelegates[serializableFileType].Invoke(fileName, fileExtension, parentFolderPath, serializableFile, out serializedFile);
        }

        public void DeserializeFile<SerializableFileType>(FileInfo serializedFile, out SerializableFileType serializableFile)
        {
            if (serializedFile == null)
            {
                throw new ArgumentNullException(nameof(serializedFile));
            }

            Type serializableFileType = typeof(SerializableFileType);

            if (!IsFileTypeDeserializationDelegateRegistered(serializableFileType))
            {
                throw new InvalidOperationException($"No file deserialization delegate registered for type '{serializableFileType}'!");
            }

            fileDeserializationDelegates[serializableFileType].Invoke(serializedFile, out object _serializableFile);
            serializableFile = (SerializableFileType)_serializableFile;
        }

        public void SerializeFolder<SerializableFolderType>(string folderName, string parentFolderPath, SerializableFolderType serializableFolder, out DirectoryInfo serializedFolder)
        {
            if (serializableFolder == null)
            {
                throw new ArgumentNullException(nameof(serializableFolder));
            }

            Type serializableFolderType = typeof(SerializableFolderType);

            if (!IsFolderTypeSerializationDelegateRegistered(serializableFolderType))
            {
                throw new InvalidOperationException($"No folder serialization delegate registered for type '{serializableFolderType}'!");
            }

            folderSerializationDelegates[serializableFolderType].Invoke(folderName, parentFolderPath, serializableFolder, out serializedFolder);
        }

        public void DeserializeFolder<SerializableFolderType>(DirectoryInfo serializedFolder, out SerializableFolderType serializableFolder)
        {
            if (serializedFolder == null)
            {
                throw new ArgumentNullException(nameof(serializedFolder));
            }

            Type serializableFolderType = typeof(SerializableFolderType);

            if (!IsFolderTypeDeserializationDelegateRegistered(serializableFolderType))
            {
                throw new InvalidOperationException($"No folder deserialization delegate registered for type '{serializableFolderType}'!");
            }

            folderDeserializationDelegates[serializableFolderType].Invoke(serializedFolder, out object _serializableFolder);
            serializableFolder = (SerializableFolderType)_serializableFolder;
        }

        public SerializableTypeMetaInfo GetSerializableTypeMetaInfo(Type type)
        {
            return serializableTypeMetaInfoCache[type];
        }
        public SerializablePrimitiveTypeMetaInfo GetSerializablePrimitiveTypeMetaInfo(Type type)
        {
            return serializablePrimitiveTypeMetaInfoCache[type];
        }
        public SerializableObjectTypeMetaInfo GetSerializableObjectTypeMetaInfo(Type type)
        {
            return serializableObjectTypeMetaInfoCache[type];
        }
        public SerializableFileTypeMetaInfo GetSerializableFileTypeMetaInfo(Type type)
        {
            return serializableFileTypeMetaInfoCache[type];
        }
        public SerializableFolderTypeMetaInfo GetSerializableFolderTypeMetaInfo(Type type)
        {
            return serializableFolderTypeMetaInfoCache[type];
        }

        public Serializability GetSerializability(Type type)
        {
            if (type == null)
            {
                throw new ArgumentNullException(nameof(type));
            }

            if (!serializabilityCache.ContainsKey(type))
            {
                throw new InvalidOperationException($"No serializability cached for type '{type}'!");
            }

            return serializabilityCache[type];
        }

        public SerializableTypeInfo GetSerializableTypeInfo(Type serializableType)
        {
            return serializableTypeInfoCache[serializableType];
        }
        public SerializablePrimitiveTypeInfo GetSerializablePrimitiveTypeInfo(Type serializablePrimitiveType)
        {
            return serializablePrimitiveTypeInfoCache[serializablePrimitiveType];
        }
        public SerializableObjectTypeInfo GetSerializableObjectTypeInfo(Type serializableObjectType)
        {
            return serializableObjectTypeInfoCache[serializableObjectType];
        }
        public SerializableFileTypeInfo GetSerializableFileTypeInfo(Type serializableFileType)
        {
            return serializableFileTypeInfoCache[serializableFileType];
        }
        public SerializableFolderTypeInfo GetSerializableFolderTypeInfo(Type serializableFolderType)
        {
            return serializableFolderTypeInfoCache[serializableFolderType];
        }

        private void CacheSerializableTypeMetaInfo(Type type, List<Type> processedTypeStack = null)
        {
            if (type == null)
            {
                throw new ArgumentNullException(nameof(type));
            }

            if (processedTypeStack == null)
            {
                processedTypeStack = new List<Type>
                {
                    type
                };
            }
            else
            {
                if (processedTypeStack.Contains(type))
                {
                    string circularPath = string.Join(" -> ", processedTypeStack.Select(_type => _type.FullName)) + " -> " + type.FullName;
                    throw new Exception($"Circular type dependency detected while caching meta info for type '{type}'! Type dependency chain: {circularPath}");
                }

                processedTypeStack.Add(type);
            }

            if (!serializableTypeMetaInfoCache.ContainsKey(type))
            {
                SerializableObjectAttribute serializableObjectAttribute = type.GetCustomAttribute<SerializableObjectAttribute>(false);
                SerializableFileAttribute serializableFileAttribute = type.GetCustomAttribute<SerializableFileAttribute>(false);
                SerializableFolderAttribute serializableFolderAttribute = type.GetCustomAttribute<SerializableFolderAttribute>(false);

                SerializableTypeMetaInfo serializableTypeMetaInfo = null;

                if (IsPrimitiveTypeSerializationDelegateRegistered(type) && IsPrimitiveTypeDeserializationDelegateRegistered(type))
                {
                    PropertyInfo[] properties = type.GetProperties();
                    FieldInfo[] fields = type.GetFields();
                    serializableTypeMetaInfo = new SerializablePrimitiveTypeMetaInfo(type);
                }
                else if (serializableFolderAttribute != null)
                {
                    PropertyInfo[] properties = type.GetProperties();
                    FieldInfo[] fields = type.GetFields();
                    bool isSerializationCompletelyOverridden = serializableObjectAttribute.OverrideSerialization && serializableObjectAttribute.OverrideDeserialization;
                    bool isSerializableTypeInfoCachingOverridden = serializableObjectAttribute.OverrideSerializableTypeInfoCaching;
                    serializableTypeMetaInfo = new SerializableObjectTypeMetaInfo(type, properties, fields, isSerializationCompletelyOverridden, isSerializableTypeInfoCachingOverridden);
                }
                else if (serializableFileAttribute != null)
                {
                    PropertyInfo[] properties = type.GetProperties();
                    FieldInfo[] fields = type.GetFields();
                    bool isSerializationCompletelyOverridden = serializableFileAttribute.OverrideSerialization && serializableFileAttribute.OverrideDeserialization;
                    bool isSerializableTypeInfoCachingOverridden = serializableFileAttribute.OverrideSerializableTypeInfoCaching;
                    serializableTypeMetaInfo = new SerializableFileTypeMetaInfo(type, properties, fields, isSerializationCompletelyOverridden, isSerializableTypeInfoCachingOverridden);
                }
                else if (serializableObjectAttribute != null)
                {
                    PropertyInfo[] properties = type.GetProperties();
                    FieldInfo[] fields = type.GetFields();
                    bool isSerializationCompletelyOverridden = serializableFolderAttribute.OverrideSerialization && serializableFolderAttribute.OverrideDeserialization;
                    bool isSerializableTypeInfoCachingOverridden = serializableFolderAttribute.OverrideSerializableTypeInfoCaching;
                    serializableTypeMetaInfo = new SerializableFolderTypeMetaInfo(type, properties, fields, isSerializationCompletelyOverridden, isSerializableTypeInfoCachingOverridden);
                }

                if (serializableTypeMetaInfo != null)
                {
                    switch (serializableTypeMetaInfo.SerializableTypeMetaInfoType)
                    {
                        case SerializableTypeMetaInfoType.Primitive:
                            serializableTypeMetaInfoCache.Add(type, serializableTypeMetaInfo);
                            serializablePrimitiveTypeMetaInfoCache.Add(type, (SerializablePrimitiveTypeMetaInfo)serializableTypeMetaInfo);
                            break;
                        case SerializableTypeMetaInfoType.Object:
                            serializableTypeMetaInfoCache.Add(type, serializableTypeMetaInfo);
                            serializableObjectTypeMetaInfoCache.Add(type, (SerializableObjectTypeMetaInfo)serializableTypeMetaInfo);
                            break;
                        case SerializableTypeMetaInfoType.File:
                            serializableTypeMetaInfoCache.Add(type, serializableTypeMetaInfo);
                            serializableObjectTypeMetaInfoCache.Add(type, (SerializableObjectTypeMetaInfo)serializableTypeMetaInfo);
                            break;
                        case SerializableTypeMetaInfoType.Folder:
                            serializableTypeMetaInfoCache.Add(type, serializableTypeMetaInfo);
                            serializableObjectTypeMetaInfoCache.Add(type, (SerializableObjectTypeMetaInfo)serializableTypeMetaInfo);
                            break;
                    }

                    foreach (Type uniqueSubType in serializableTypeMetaInfo.UniqueSubTypes)
                    {
                        CacheSerializableTypeMetaInfo(uniqueSubType, processedTypeStack);
                    }
                }
            }
        }

        private void CacheSerializability(Type potentiallySerializableType, SerializableTypeMetaInfo serializableTypeMetaInfo)
        {
            if (potentiallySerializableType == null)
            {
                throw new ArgumentNullException(nameof(potentiallySerializableType));
            }

            Serializability serializability = Serializability.None;

            if (!serializabilityCache.TryGetValue(potentiallySerializableType, out serializability))
            {
                if ((potentiallySerializableType.IsPublic || potentiallySerializableType.IsNestedPublic) &&
                    !potentiallySerializableType.IsAbstract &&
                    (potentiallySerializableType.IsClass || potentiallySerializableType.IsValueType || potentiallySerializableType.IsEnum) &&
                    serializableTypeMetaInfo != null &&
                    (potentiallySerializableType.GetConstructor(Type.EmptyTypes) != null || serializableTypeMetaInfo.IsSerializationCompletelyOverridden))
                {
                    switch (serializableTypeMetaInfo.SerializableTypeMetaInfoType)
                    {
                        case SerializableTypeMetaInfoType.Primitive:
                            serializability = Serializability.Primitive;
                            break;
                        case SerializableTypeMetaInfoType.Object:
                            serializability = Serializability.Object;
                            break;
                        case SerializableTypeMetaInfoType.File:
                            serializability = Serializability.File;
                            break;
                        case SerializableTypeMetaInfoType.Folder:
                            serializability = Serializability.Folder;
                            break;
                    }
                }
            }

            serializabilityCache.Add(potentiallySerializableType, serializability);

            switch (serializability)
            {
                case Serializability.None:
                    unserializableTypes.Add(potentiallySerializableType);
                    break;
                case Serializability.Primitive:
                    primitiveTypeInfos.Add(potentiallySerializableType);
                    break;
                case Serializability.Object:
                    objectTypes.Add(potentiallySerializableType);
                    break;
                case Serializability.File:
                    fileTypes.Add(potentiallySerializableType);
                    break;
                case Serializability.Folder:
                    folderTypes.Add(potentiallySerializableType);
                    break;
            }

            if (serializability != Serializability.None)
            {
                foreach (Type uniqueSubType in serializableTypeMetaInfo.UniqueSubTypes)
                {
                    CacheSerializability(uniqueSubType, serializableTypeMetaInfo);
                }
            }
        }

        private void CacheSerializablePrimitiveTypeInfo(Type serializablePrimitiveType, SerializablePrimitiveTypeMetaInfo serializablePrimitiveTypeMetaInfo)
        {
            SerializablePrimitiveTypeInfo serializablePrimitiveTypeInfo = new SerializablePrimitiveTypeInfo(serializablePrimitiveType);

            serializableTypeInfoCache.Add(serializablePrimitiveType, serializablePrimitiveTypeInfo);
            serializablePrimitiveTypeInfoCache.Add(serializablePrimitiveType, serializablePrimitiveTypeInfo);
        }

        private void CacheSerializableObjectTypeInfo(Type serializableObjectType, SerializableObjectTypeMetaInfo serializableObjectTypeMetaInfo)
        {
            SerializableObjectTypeInfo serializableObjectTypeInfo = null;

            if (serializableObjectTypeMetaInfo.IsSerializableTypeInfoCachingOverridden)
            {
                // TODO: return the type info from the static override method from the type
                // TODO: Also ensure that the generic type definitions without provided generic type parameters, in the initial type definitions, are never checked for serializability or anything like that
            }
            else
            {
                HashSet<SerializablePrimitiveTypeInfo> subSerializablePrimitiveTypeInfos = new HashSet<SerializablePrimitiveTypeInfo>();
                HashSet<SerializableObjectTypeInfo> subSerializableObjectTypeInfos = new HashSet<SerializableObjectTypeInfo>();

                foreach (Type uniqueSubType in serializableObjectTypeMetaInfo.UniqueSubTypes)
                {
                    Serializability uniqueSubTypeSerializability = GetSerializability(uniqueSubType);
                    switch (uniqueSubTypeSerializability)
                    {
                        case Serializability.Primitive:
                            SerializablePrimitiveTypeInfo subSerializablePrimitiveTypeInfo;
                            if (!serializablePrimitiveTypeInfoCache.TryGetValue(uniqueSubType, out subSerializablePrimitiveTypeInfo))
                            {
                                CacheSerializablePrimitiveTypeInfo(uniqueSubType, GetSerializablePrimitiveTypeMetaInfo(uniqueSubType));
                            }
                            else
                            {
                                subSerializablePrimitiveTypeInfo = GetSerializablePrimitiveTypeInfo(uniqueSubType);
                            }
                            subSerializablePrimitiveTypeInfos.Add(subSerializablePrimitiveTypeInfo);
                            break;
                        case Serializability.Object:
                            SerializableObjectTypeInfo subSerializableObjectTypeInfo;
                            if (!serializableObjectTypeInfoCache.TryGetValue(uniqueSubType, out subSerializableObjectTypeInfo))
                            {
                                CacheSerializableObjectTypeInfo(uniqueSubType, GetSerializableObjectTypeMetaInfo(uniqueSubType));
                            }
                            else
                            {
                                subSerializableObjectTypeInfo = GetSerializableObjectTypeInfo(uniqueSubType);
                            }
                            subSerializableObjectTypeInfos.Add(subSerializableObjectTypeInfo);
                            break;
                        case Serializability.File:
                            throw new InvalidOperationException($"Serializable object types cannot contain serializable file types! Serializable object type: '{serializableObjectType}'");
                        case Serializability.Folder:
                            throw new InvalidOperationException($"Serializable object types cannot contain serializable folder types! Serializable object type: '{serializableObjectType}'");
                    }
                }

                serializableObjectTypeInfo = new SerializableObjectTypeInfo(serializableObjectType, subSerializablePrimitiveTypeInfos, subSerializableObjectTypeInfos);
            }

            serializableTypeInfoCache.Add(serializableObjectType, serializableObjectTypeInfo);
            serializableObjectTypeInfoCache.Add(serializableObjectType, serializableObjectTypeInfo);
        }

        private void CacheSerializableFileTypeInfo(Type serializableFileType, SerializableFileTypeMetaInfo serializableFileTypeMetaInfo)
        {
            SerializableFileTypeInfo serializableFileTypeInfo = null;

            if (serializableFileTypeMetaInfo.IsSerializableTypeInfoCachingOverridden)
            {
                // TODO: return the type info from the static override method from the type
                // TODO: Also ensure that the generic type definitions without provided generic type parameters, in the initial type definitions, are never checked for serializability or anything like that
            }
            else
            {
                HashSet<SerializableObjectTypeInfo> subSerializableObjectTypeInfos = new HashSet<SerializableObjectTypeInfo>();

                foreach (Type uniqueSubType in serializableFileTypeMetaInfo.UniqueSubTypes)
                {
                    Serializability uniqueSubTypeSerializability = GetSerializability(uniqueSubType);
                    switch (uniqueSubTypeSerializability)
                    {
                        case Serializability.Primitive:
                            throw new InvalidOperationException($"Serializable file types cannot contain serializable primitive types! Serializable file type: '{serializableFileType}'");
                        case Serializability.Object:
                            SerializableObjectTypeInfo subSerializableObjectTypeInfo;
                            if (!serializableObjectTypeInfoCache.TryGetValue(uniqueSubType, out subSerializableObjectTypeInfo))
                            {
                                CacheSerializableObjectTypeInfo(uniqueSubType, GetSerializableObjectTypeMetaInfo(uniqueSubType));
                            }
                            else
                            {
                                subSerializableObjectTypeInfo = GetSerializableObjectTypeInfo(uniqueSubType);
                            }
                            subSerializableObjectTypeInfos.Add(subSerializableObjectTypeInfo);
                            break;
                        case Serializability.File:
                            throw new InvalidOperationException($"Serializable file types cannot contain serializable file types! Serializable file type: '{serializableFileType}'");
                        case Serializability.Folder:
                            throw new InvalidOperationException($"Serializable file types cannot contain serializable folder types! Serializable file type: '{serializableFileType}'");
                    }
                }

                serializableFileTypeInfo = new SerializableFileTypeInfo(serializableFileType, subSerializableObjectTypeInfos);
            }

            serializableTypeInfoCache.Add(serializableFileType, serializableFileTypeInfo);
            serializableFileTypeInfoCache.Add(serializableFileType, serializableFileTypeInfo);
        }

        private void CacheSerializableFolderTypeInfo(Type serializableFolderType, SerializableFolderTypeMetaInfo serializableFolderTypeMetaInfo)
        {
            SerializableFolderTypeInfo serializableFolderTypeInfo = null;

            if (serializableFolderTypeMetaInfo.IsSerializableTypeInfoCachingOverridden)
            {
                // TODO: return the type info from the static override method from the type
                // TODO: Also ensure that the generic type definitions without provided generic type parameters, in the initial type definitions, are never checked for serializability or anything like that
            }
            else
            {
                HashSet<SerializableFileTypeInfo> subSerializableFileTypeInfos = new HashSet<SerializableFileTypeInfo>();
                HashSet<SerializableFolderTypeInfo> subSerializableFolderTypeInfos = new HashSet<SerializableFolderTypeInfo>();

                foreach (Type uniqueSubType in serializableFolderTypeMetaInfo.UniqueSubTypes)
                {
                    Serializability uniqueSubTypeSerializability = GetSerializability(uniqueSubType);
                    switch (uniqueSubTypeSerializability)
                    {
                        case Serializability.Primitive:
                            throw new InvalidOperationException($"Serializable folder types cannot contain serializable primitive types! Serializable folder type: '{serializableFolderType}'");
                        case Serializability.Object:
                            throw new InvalidOperationException($"Serializable folder types cannot contain serializable object types! Serializable folder type: '{serializableFolderType}'");
                        case Serializability.File:
                            SerializableFileTypeInfo subSerializableFileTypeInfo;
                            if (!serializableFileTypeInfoCache.TryGetValue(uniqueSubType, out subSerializableFileTypeInfo))
                            {
                                CacheSerializableFileTypeInfo(uniqueSubType, GetSerializableFileTypeMetaInfo(uniqueSubType));
                            }
                            else
                            {
                                subSerializableFileTypeInfo = GetSerializableFileTypeInfo(uniqueSubType);
                            }
                            subSerializableFileTypeInfos.Add(subSerializableFileTypeInfo);
                            break;
                        case Serializability.Folder:
                            SerializableFolderTypeInfo subSerializableFolderTypeInfo;
                            if (!serializableFolderTypeInfoCache.TryGetValue(uniqueSubType, out subSerializableFolderTypeInfo))
                            {
                                CacheSerializableFolderTypeInfo(uniqueSubType, GetSerializableFolderTypeMetaInfo(uniqueSubType));
                            }
                            else
                            {
                                subSerializableFolderTypeInfo = GetSerializableFolderTypeInfo(uniqueSubType);
                            }
                            subSerializableFolderTypeInfos.Add(subSerializableFolderTypeInfo);
                            break;
                    }
                }

                serializableFolderTypeInfo = new SerializableFolderTypeInfo(serializableFolderType, subSerializableFileTypeInfos, subSerializableFolderTypeInfos);
            }

            serializableTypeInfoCache.Add(serializableFolderType, serializableFolderTypeInfo);
            serializableFolderTypeInfoCache.Add(serializableFolderType, serializableFolderTypeInfo);
        }

        // TODO: Optimize
        private List<List<SerializableObjectTypeInfo>> GetSortedObjectTypeInfos()
        {
            List<SerializableObjectTypeInfo> serializableObjectTypeDefinitionInfos = serializableObjectTypeInfoCache.Values.ToList();

            List<List<SerializableObjectTypeInfo>> serializableObjectTypeQueues = new List<List<SerializableObjectTypeInfo>>();
            HashSet<Type> allProcessedTypes = new HashSet<Type>();

            int lastCount = serializableObjectTypeDefinitionInfos.Count;
            while (serializableObjectTypeDefinitionInfos.Count > 0)
            {
                HashSet<Type> processedTypes = new HashSet<Type>();
                List<SerializableObjectTypeInfo> serializableObjectTypeQueue = new List<SerializableObjectTypeInfo>();
                List<SerializableObjectTypeInfo> toRemove = new List<SerializableObjectTypeInfo>();

                foreach (SerializableObjectTypeInfo serializableObjectTypeDefinitionInfo in serializableObjectTypeDefinitionInfos)
                {
                    bool allSubTypesProcessed = true;
                    foreach (SerializableObjectTypeInfo subTypeInfo in serializableObjectTypeDefinitionInfo.SubSerializableObjectTypeInfos)
                    {
                        if (!allProcessedTypes.Contains(subTypeInfo.SerializableObjectType))
                        {
                            allSubTypesProcessed = false;
                            break;
                        }
                    }

                    if (allSubTypesProcessed)
                    {
                        serializableObjectTypeQueue.Add(serializableObjectTypeDefinitionInfo);
                        processedTypes.Add(serializableObjectTypeDefinitionInfo.SerializableObjectType);
                        toRemove.Add(serializableObjectTypeDefinitionInfo);
                    }
                }

                foreach (Type processedType in processedTypes)
                {
                    allProcessedTypes.Add(processedType);
                }

                foreach (SerializableObjectTypeInfo item in toRemove)
                {
                    serializableObjectTypeDefinitionInfos.Remove(item);
                }

                if (serializableObjectTypeQueue.Count > 0)
                {
                    serializableObjectTypeQueues.Add(serializableObjectTypeQueue);
                }

                if (lastCount == serializableObjectTypeDefinitionInfos.Count)
                {
                    throw new InfiniteLoopException();
                }

                lastCount = serializableObjectTypeDefinitionInfos.Count;
            }

            return serializableObjectTypeQueues;
        }

        // TODO: Optimize
        private List<List<SerializableFolderTypeInfo>> GetSortedFolderTypeInfos()
        {
            List<SerializableFolderTypeInfo> serializableFolderTypeDefinitionInfos = serializableFolderTypeInfoCache.Values.ToList();

            List<List<SerializableFolderTypeInfo>> serializableFolderTypeQueues = new List<List<SerializableFolderTypeInfo>>();
            HashSet<Type> allProcessedTypes = new HashSet<Type>();

            int lastCount = serializableFolderTypeDefinitionInfos.Count;
            while (serializableFolderTypeDefinitionInfos.Count > 0)
            {
                HashSet<Type> processedTypes = new HashSet<Type>();
                List<SerializableFolderTypeInfo> serializableFolderTypeQueue = new List<SerializableFolderTypeInfo>();
                List<SerializableFolderTypeInfo> toRemove = new List<SerializableFolderTypeInfo>();

                foreach (SerializableFolderTypeInfo serializableFolderTypeDefinitionInfo in serializableFolderTypeDefinitionInfos)
                {
                    bool allSubTypesProcessed = true;
                    foreach (SerializableFolderTypeInfo subTypeInfo in serializableFolderTypeDefinitionInfo.SubSerializableFolderTypeInfos)
                    {
                        if (!allProcessedTypes.Contains(subTypeInfo.SerializableFolderType))
                        {
                            allSubTypesProcessed = false;
                            break;
                        }
                    }

                    if (allSubTypesProcessed)
                    {
                        serializableFolderTypeQueue.Add(serializableFolderTypeDefinitionInfo);
                        processedTypes.Add(serializableFolderTypeDefinitionInfo.SerializableFolderType);
                        toRemove.Add(serializableFolderTypeDefinitionInfo);
                    }
                }

                foreach (Type processedType in processedTypes)
                {
                    allProcessedTypes.Add(processedType);
                }

                foreach (SerializableFolderTypeInfo item in toRemove)
                {
                    serializableFolderTypeDefinitionInfos.Remove(item);
                }

                if (serializableFolderTypeQueue.Count > 0)
                {
                    serializableFolderTypeQueues.Add(serializableFolderTypeQueue);
                }

                if (lastCount == serializableFolderTypeDefinitionInfos.Count)
                {
                    throw new InfiniteLoopException();
                }

                lastCount = serializableFolderTypeDefinitionInfos.Count;
            }

            return serializableFolderTypeQueues;
        }

        // TODO: Integrate into updated framework (utilize the cached typeMetaInfos and typeInfos)
        // TODO: Split into manageable chunks (chunks = methods)
        private void CacheObjectSerializationDelegates(SerializableObjectTypeInfo serializableObjectTypeInfo)
        {
            Type serializableObjectType = serializableObjectTypeInfo.SerializableObjectType;
            SerializableObjectTypeMetaInfo serializableObjectTypeMetaInfo = serializableObjectTypeMetaInfoCache[serializableObjectType];

            SerializableObjectAttribute serializableObjectAttribute = serializableObjectType.GetCustomAttribute<SerializableObjectAttribute>();

            bool overrideSerialization = serializableObjectAttribute.OverrideSerialization;
            bool overrideDeserialization = serializableObjectAttribute.OverrideDeserialization;

            if (overrideSerialization)
            {
                MethodInfo methodInfo = serializableObjectType.GetMethod("SerializeObject", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(string), typeof(object), typeof(XElement).MakeByRefType() }, null);

                if (methodInfo == null)
                {
                    throw new Exception($"Type '{serializableObjectType}' is marked as overriding the defualt serialization behaviour, but it does not implement a method with the signature 'public static void SerializeObject(string listName, object _object, out XElement serializedList)'!");
                }

                objectSerializationDelegates.Add(serializableObjectType, (SerializeObjectDelegate)methodInfo.CreateDelegate(typeof(SerializeObjectDelegate)));
            }
            if (overrideDeserialization)
            {
                MethodInfo methodInfo = serializableObjectType.GetMethod("DeserializeObject", BindingFlags.Public | BindingFlags.Static, null, new Type[] { typeof(XElement), typeof(object).MakeByRefType() }, null);

                if (methodInfo == null)
                {
                    throw new Exception($"Type '{serializableObjectType}' is marked as overriding the defualt deserialization behaviour, but it does not implement a method with the signature 'public static void DeserializeObject(XElement serializedList, out object _object)'!");
                }

                objectDeserializationDelegates.Add(serializableObjectType, (DeserializeObjectDelegate)methodInfo.CreateDelegate(typeof(DeserializeObjectDelegate)));
            }
            if (overrideSerialization && overrideDeserialization)
            {
                return;
            }

            PropertyInfo[] properties = serializableObjectTypeMetaInfo.Properties;
            FieldInfo[] fields = serializableObjectTypeMetaInfo.Fields;

            Dictionary<string, SerializePrimitiveDelegate> serializePrimitivePropertyDelegates = new Dictionary<string, SerializePrimitiveDelegate>();
            Dictionary<string, DeserializePrimitiveDelegate> deserializePrimitivePropertyDelegates = new Dictionary<string, DeserializePrimitiveDelegate>();

            Dictionary<string, SerializeObjectDelegate> serializeObjectPropertyDelegates = new Dictionary<string, SerializeObjectDelegate>();
            Dictionary<string, DeserializeObjectDelegate> deserializeObjectPropertyDelegates = new Dictionary<string, DeserializeObjectDelegate>();

            Dictionary<string, SerializePrimitiveDelegate> serializePrimitiveFieldDelegates = new Dictionary<string, SerializePrimitiveDelegate>();
            Dictionary<string, DeserializePrimitiveDelegate> deserializePrimitiveFieldDelegates = new Dictionary<string, DeserializePrimitiveDelegate>();

            Dictionary<string, SerializeObjectDelegate> serializeObjectFieldDelegates = new Dictionary<string, SerializeObjectDelegate>();
            Dictionary<string, DeserializeObjectDelegate> deserializeObjectFieldDelegates = new Dictionary<string, DeserializeObjectDelegate>();

            Dictionary<Type, SerializePrimitiveDelegate> primitiveSerializationSubDelegateDictionary = primitiveSerializationSubDelegateDictionaries[serializableObjectType];
            Dictionary<Type, DeserializePrimitiveDelegate> primitiveDeserializationSubDelegateDictionary = primitiveDeserializationSubDelegateDictionaries[serializableObjectType];

            Dictionary<Type, SerializeObjectDelegate> objectSerializationSubDelegateDictionary = objectSerializationSubDelegateDictionaries[serializableObjectType];
            Dictionary<Type, DeserializeObjectDelegate> objectDeserializationSubDelegateDictionary = objectDeserializationSubDelegateDictionaries[serializableObjectType];

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
                                throw new Exception($"No primitive serialization delegate registered for type '{propertyType}'!");
                            }

                            serializePrimitivePropertyDelegates.Add(property.Name, primitiveSerializationSubDelegateDictionary[propertyType]);
                        }
                        if (!overrideDeserialization)
                        {
                            if (!IsPrimitiveTypeDeserializationDelegateRegistered(propertyType))
                            {
                                throw new Exception($"No primitive deserialization delegate registered for type '{propertyType}'!");
                            }

                            deserializePrimitivePropertyDelegates.Add(property.Name, primitiveDeserializationSubDelegateDictionary[propertyType]);
                        }
                        break;
                    case Serializability.Object:
                        if (!overrideSerialization)
                        {
                            if (!IsObjectTypeSerializationDelegateRegistered(propertyType))
                            {
                                throw new Exception($"No object serialization delegate registered for type '{propertyType}'!");
                            }

                            serializeObjectPropertyDelegates.Add(property.Name, objectSerializationSubDelegateDictionary[propertyType]);
                        }
                        if (!overrideDeserialization)
                        {
                            if (!IsObjectTypeDeserializationDelegateRegistered(propertyType))
                            {
                                throw new Exception($"No object deserialization delegate registered for type '{propertyType}'!");
                            }

                            deserializeObjectPropertyDelegates.Add(property.Name, objectDeserializationSubDelegateDictionary[propertyType]);
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
                                throw new Exception($"No primitive serialization delegate registered for type '{fieldType}'!");
                            }
                            serializePrimitiveFieldDelegates.Add(field.Name, primitiveSerializationSubDelegateDictionary[fieldType]);
                        }
                        if (!overrideDeserialization)
                        {
                            if (!IsPrimitiveTypeDeserializationDelegateRegistered(fieldType))
                            {
                                throw new Exception($"No primitive deserialization delegate registered for type '{fieldType}'!");
                            }
                            deserializePrimitiveFieldDelegates.Add(field.Name, primitiveDeserializationSubDelegateDictionary[fieldType]);
                        }
                        break;
                    case Serializability.Object:
                        if (!overrideSerialization)
                        {
                            if (!IsObjectTypeSerializationDelegateRegistered(fieldType))
                            {
                                throw new Exception($"No object serialization delegate registered for type '{fieldType}'!");
                            }

                            serializeObjectFieldDelegates.Add(field.Name, objectSerializationSubDelegateDictionary[fieldType]);
                        }
                        if (!overrideDeserialization)
                        {
                            if (!IsObjectTypeDeserializationDelegateRegistered(fieldType))
                            {
                                throw new Exception($"No object deserialization delegate registered for type '{fieldType}'!");
                            }

                            deserializeObjectFieldDelegates.Add(field.Name, objectDeserializationSubDelegateDictionary[fieldType]);
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

                    PropertyInfo[] properties = serializableObjectType.GetProperties();
                    FieldInfo[] fields = serializableObjectType.GetFields();

                    foreach (PropertyInfo property in properties)
                    {
                        Type propertyType = property.PropertyType;
                        Serializability propertySerializability = GetSerializability(propertyType);
                        string propertyName = property.Name;

                        switch (propertySerializability)
                        {
                            case Serializability.Primitive:
                                serializePrimitivePropertyDelegates[propertyName].Invoke(propertyName, property.GetValue(_object), out XAttribute serializedPrimitiveProperty);
                                serializedObject.Add(serializedPrimitiveProperty);
                                break;
                            case Serializability.Object:
                                serializeObjectPropertyDelegates[propertyName].Invoke(propertyName, property.GetValue(_object), out XElement serializedObjectProperty);
                                serializedObject.Add(serializedObjectProperty);
                                break;
                        }
                    }

                    foreach (FieldInfo field in fields)
                    {
                        Type fieldType = field.FieldType;
                        Serializability fieldSerializability = GetSerializability(fieldType);
                        string fieldName = field.Name;

                        switch (fieldSerializability)
                        {
                            case Serializability.Primitive:
                                serializePrimitiveFieldDelegates[fieldName].Invoke(fieldName, field.GetValue(_object), out XAttribute serializedPrimitiveField);
                                serializedObject.Add(serializedPrimitiveField);
                                break;
                            case Serializability.Object:
                                serializeObjectFieldDelegates[fieldName].Invoke(fieldName, field.GetValue(_object), out XElement serializedObjectField);
                                serializedObject.Add(serializedObjectField);
                                break;
                        }
                    }
                });
            }
            if (!overrideDeserialization)
            {
                objectDeserializationDelegates.Add(serializableObjectType, (XElement serializedObject, out object _object) =>
                {
                    _object = Activator.CreateInstance(serializableObjectType);

                    PropertyInfo[] properties = serializableObjectType.GetProperties();
                    FieldInfo[] fields = serializableObjectType.GetFields();

                    XAttribute[] serializedPrimitives = serializedObject.Attributes().ToArray();
                    XElement[] serializedObjects = serializedObject.Elements().ToArray();

                    foreach (KeyValuePair<string, DeserializePrimitiveDelegate> propertyNamePrimitiveDeserializeDelegatePair in deserializePrimitivePropertyDelegates)
                    {
                        PropertyInfo primitiveProperty = properties.FirstOrDefault(property => property.Name == propertyNamePrimitiveDeserializeDelegatePair.Key);
                        XAttribute serializedPrimitiveProperty = serializedPrimitives.FirstOrDefault(_serializedPrimitive => _serializedPrimitive.Name == propertyNamePrimitiveDeserializeDelegatePair.Key);

                        if (primitiveProperty == null)
                        {
                            throw new Exception($"Primitive property '{propertyNamePrimitiveDeserializeDelegatePair.Key}' not found in object type '{serializableObjectType}'!");
                        }
                        if (serializedPrimitiveProperty == null)
                        {
                            throw new Exception($"Primitive roperty '{propertyNamePrimitiveDeserializeDelegatePair.Key}' not found in serialized object '{serializedObject.Name}'!");
                        }

                        string primitivePropertyName = primitiveProperty.Name;

                        deserializePrimitivePropertyDelegates[primitivePropertyName].Invoke(serializedPrimitiveProperty, out object primitivePropertyValue);
                        primitiveProperty.SetValue(_object, primitivePropertyValue);
                    }

                    foreach (KeyValuePair<string, DeserializeObjectDelegate> propertyNameObjectDeserializeDelegatePair in deserializeObjectPropertyDelegates)
                    {
                        PropertyInfo objectProperty = properties.FirstOrDefault(property => property.Name == propertyNameObjectDeserializeDelegatePair.Key);
                        XElement serializedObjectProperty = serializedObjects.FirstOrDefault(_serializedObject => _serializedObject.Name == propertyNameObjectDeserializeDelegatePair.Key);

                        if (objectProperty == null)
                        {
                            throw new Exception($"Object property '{propertyNameObjectDeserializeDelegatePair.Key}' not found in object type '{serializableObjectType}'!");
                        }
                        if (serializedObjectProperty == null)
                        {
                            throw new Exception($"Object property '{propertyNameObjectDeserializeDelegatePair.Key}' not found in serialized object '{serializedObjectProperty.Name}'!");
                        }

                        string objectPropertyName = objectProperty.Name;

                        deserializeObjectPropertyDelegates[objectPropertyName].Invoke(serializedObjectProperty, out object objectPropertyValue);
                        objectProperty.SetValue(_object, objectPropertyValue);
                    }

                    foreach (KeyValuePair<string, DeserializePrimitiveDelegate> fieldNamePrimitiveDeserializeDelegatePair in deserializePrimitiveFieldDelegates)
                    {
                        FieldInfo primitiveField = fields.FirstOrDefault(field => field.Name == fieldNamePrimitiveDeserializeDelegatePair.Key);
                        XAttribute serializedPrimitiveField = serializedPrimitives.FirstOrDefault(_serializedPrimitive => _serializedPrimitive.Name == fieldNamePrimitiveDeserializeDelegatePair.Key);

                        if (primitiveField == null)
                        {
                            throw new Exception($"Primitive field '{fieldNamePrimitiveDeserializeDelegatePair.Key}' not found in object type '{serializableObjectType}'!");
                        }
                        if (serializedPrimitiveField == null)
                        {
                            throw new Exception($"Primitive field '{fieldNamePrimitiveDeserializeDelegatePair.Key}' not found in serialized object '{serializedObject.Name}'!");
                        }

                        string primitiveFieldName = primitiveField.Name;

                        deserializePrimitiveFieldDelegates[primitiveFieldName].Invoke(serializedPrimitiveField, out object primitiveFieldValue);
                        primitiveField.SetValue(_object, primitiveFieldValue);
                    }

                    foreach (KeyValuePair<string, DeserializeObjectDelegate> fieldNameObjectDeserializeDelegatePair in deserializeObjectFieldDelegates)
                    {
                        FieldInfo objectField = fields.FirstOrDefault(field => field.Name == fieldNameObjectDeserializeDelegatePair.Key);
                        XElement serializedObjectField = serializedObjects.FirstOrDefault(_serializedObject => _serializedObject.Name == fieldNameObjectDeserializeDelegatePair.Key);

                        if (objectField == null)
                        {
                            throw new Exception($"Object field '{fieldNameObjectDeserializeDelegatePair.Key}' not found in object type '{serializableObjectType}'!");
                        }
                        if (serializedObjectField == null)
                        {
                            throw new Exception($"Object field '{fieldNameObjectDeserializeDelegatePair.Key}' not found in serialized object '{serializedObjectField.Name}'!");
                        }

                        string objectFieldName = objectField.Name;

                        deserializeObjectFieldDelegates[objectFieldName].Invoke(serializedObjectField, out object objectFieldValue);
                        objectField.SetValue(_object, objectFieldValue);
                    }
                });
            }
        }

        // TODO: Implement
        private void CacheFileSerializationDelegates(SerializableFileTypeInfo serializableFileTypeInfo)
        {

        }

        // TODO: Implement
        private void CacheFolderSerializationDelegates(SerializableFolderTypeInfo serializableFolderTypeInfo)
        {

        }

        private void CacheObjectSerializationSubDelegates(SerializableObjectTypeInfo serializableObjectTypeInfo)
        {
            Type serializableObjectType = serializableObjectTypeInfo.SerializableObjectType;

            if (primitiveSerializationSubDelegateDictionaries.ContainsKey(serializableObjectType))
            {
                throw new Exception($"The primitive serialization sub delegates for object type '{serializableObjectType}' are already registered!");
            }
            if (primitiveDeserializationSubDelegateDictionaries.ContainsKey(serializableObjectType))
            {
                throw new Exception($"The primitive deserialization sub delegates for object type '{serializableObjectType}' are already registered!");
            }

            if (objectSerializationSubDelegateDictionaries.ContainsKey(serializableObjectType))
            {
                throw new Exception($"The object serialization sub delegates for object type '{serializableObjectType}' are already registered!");
            }
            if (objectDeserializationSubDelegateDictionaries.ContainsKey(serializableObjectType))
            {
                throw new Exception($"The object deserialization sub delegates for object type '{serializableObjectType}' are already registered!");
            }

            Dictionary<Type, SerializePrimitiveDelegate> primitiveSerializationSubDelegates = new Dictionary<Type, SerializePrimitiveDelegate>();
            Dictionary<Type, DeserializePrimitiveDelegate> primitiveDeserializationSubDelegates = new Dictionary<Type, DeserializePrimitiveDelegate>();

            Dictionary<Type, SerializeObjectDelegate> objectSerializationSubDelegates = new Dictionary<Type, SerializeObjectDelegate>();
            Dictionary<Type, DeserializeObjectDelegate> objectDeserializationSubDelegates = new Dictionary<Type, DeserializeObjectDelegate>();

            foreach (SerializablePrimitiveTypeInfo subSerializablePrimitiveTypeInfo in serializableObjectTypeInfo.SubSerializablePrimitiveTypeInfos)
            {
                Type subSerializablePrimitiveType = subSerializablePrimitiveTypeInfo.SerializablePrimitiveType;
                if (primitiveSerializationSubDelegates.ContainsKey(subSerializablePrimitiveType))
                {
                    throw new Exception($"The primitive serialization delegate for primitive type '{subSerializablePrimitiveType}' is already registered as sub delegate for object type '{subSerializablePrimitiveType}'!");
                }
                if (primitiveDeserializationSubDelegates.ContainsKey(subSerializablePrimitiveType))
                {
                    throw new Exception($"The primitive deserialization delegate for primitive type '{subSerializablePrimitiveType}' is already registered as sub delegate for object type '{subSerializablePrimitiveType}'!");
                }

                primitiveSerializationSubDelegates.Add(subSerializablePrimitiveType, primitiveSerializationDelegates[subSerializablePrimitiveType]);
                primitiveDeserializationSubDelegates.Add(subSerializablePrimitiveType, primitiveDeserializationDelegates[subSerializablePrimitiveType]);
            }

            foreach (SerializableObjectTypeInfo subSerializableObjectTypeInfo in serializableObjectTypeInfo.SubSerializableObjectTypeInfos)
            {
                Type subSerializableObjectType = subSerializableObjectTypeInfo.SerializableObjectType;
                if (objectSerializationSubDelegates.ContainsKey(subSerializableObjectType))
                {
                    throw new Exception($"The object serialization delegate for object type '{subSerializableObjectType}' is already registered as sub delegate for object type '{serializableObjectType}'!");
                }
                if (objectDeserializationSubDelegates.ContainsKey(subSerializableObjectType))
                {
                    throw new Exception($"The object deserialization delegate for object type '{subSerializableObjectType}' is already registered as sub delegate for object type '{serializableObjectType}'!");
                }

                objectSerializationSubDelegates.Add(subSerializableObjectType, objectSerializationDelegates[subSerializableObjectType]);
                objectDeserializationSubDelegates.Add(subSerializableObjectType, objectDeserializationDelegates[subSerializableObjectType]);
            }

            primitiveSerializationSubDelegateDictionaries.Add(serializableObjectType, primitiveSerializationSubDelegates);
            primitiveDeserializationSubDelegateDictionaries.Add(serializableObjectType, primitiveDeserializationSubDelegates);

            objectSerializationSubDelegateDictionaries.Add(serializableObjectType, objectSerializationSubDelegates);
            objectDeserializationSubDelegateDictionaries.Add(serializableObjectType, objectDeserializationSubDelegates);
        }

        private void CacheFileSerializationSubDelegates(SerializableFileTypeInfo serializableFileTypeInfo)
        {
            Type serializableFileType = serializableFileTypeInfo.SerializableFileType;

            if (objectSerializationSubDelegateDictionaries.ContainsKey(serializableFileType))
            {
                throw new Exception($"The object serialization sub delegates for file type '{serializableFileType}' are already registered!");
            }
            if (objectDeserializationSubDelegateDictionaries.ContainsKey(serializableFileType))
            {
                throw new Exception($"The object deserialization sub delegates for file type '{serializableFileType}' are already registered!");
            }

            Dictionary<Type, SerializeObjectDelegate> objectSerializationSubDelegates = new Dictionary<Type, SerializeObjectDelegate>();
            Dictionary<Type, DeserializeObjectDelegate> objectDeserializationSubDelegates = new Dictionary<Type, DeserializeObjectDelegate>();

            foreach (SerializableObjectTypeInfo subSerializableObjectTypeInfo in serializableFileTypeInfo.SubSerializableObjectTypeInfos)
            {
                Type subSerializableObjectType = subSerializableObjectTypeInfo.SerializableObjectType;
                if (objectSerializationSubDelegates.ContainsKey(subSerializableObjectType))
                {
                    throw new Exception($"The object serialization sub delegate for object type '{subSerializableObjectType}' is already registered!");
                }
                if (objectDeserializationSubDelegates.ContainsKey(subSerializableObjectType))
                {
                    throw new Exception($"The object deserialization sub delegate for object type '{subSerializableObjectType}' is already registered!");
                }

                objectSerializationSubDelegates.Add(subSerializableObjectType, objectSerializationDelegates[subSerializableObjectType]);
                objectDeserializationSubDelegates.Add(subSerializableObjectType, objectDeserializationDelegates[subSerializableObjectType]);
            }

            objectSerializationSubDelegateDictionaries.Add(serializableFileType, objectSerializationSubDelegates);
            objectDeserializationSubDelegateDictionaries.Add(serializableFileType, objectDeserializationSubDelegates);
        }

        private void CacheFolderSerializationSubDelegates(SerializableFolderTypeInfo serializableFolderTypeInfo)
        {
            Type serializableFolderType = serializableFolderTypeInfo.SerializableFolderType;

            if (fileSerializationSubDelegateDictionaries.ContainsKey(serializableFolderType))
            {
                throw new Exception($"The file serialization sub delegates for folder type '{serializableFolderType}' are already registered!");
            }
            if (fileDeserializationSubDelegateDictionaries.ContainsKey(serializableFolderType))
            {
                throw new Exception($"The file deserialization sub delegates for folder type '{serializableFolderType}' are already registered!");
            }

            if (folderSerializationSubDelegateDictionaries.ContainsKey(serializableFolderType))
            {
                throw new Exception($"The folder serialization sub delegates for folder type '{serializableFolderType}' are already registered!");
            }
            if (folderDeserializationSubDelegateDictionaries.ContainsKey(serializableFolderType))
            {
                throw new Exception($"The folder deserialization sub delegates for folder type '{serializableFolderType}' are already registered!");
            }

            Dictionary<Type, SerializeFileDelegate> fileSerializationSubDelegates = new Dictionary<Type, SerializeFileDelegate>();
            Dictionary<Type, DeserializeFileDelegate> fileDeserializationSubDelegates = new Dictionary<Type, DeserializeFileDelegate>();

            Dictionary<Type, SerializeFolderDelegate> folderSerializationSubDelegates = new Dictionary<Type, SerializeFolderDelegate>();
            Dictionary<Type, DeserializeFolderDelegate> folderDeserializationSubDelegates = new Dictionary<Type, DeserializeFolderDelegate>();

            foreach (SerializableFileTypeInfo subSerializableFileTypeInfo in serializableFolderTypeInfo.SubSerializableFileTypeInfos)
            {
                Type subSerializableFileType = subSerializableFileTypeInfo.SerializableFileType;
                if (fileSerializationSubDelegates.ContainsKey(subSerializableFileType))
                {
                    throw new Exception($"The file serialization delegate for file type '{subSerializableFileType}' is already registered as sub delegate for folder type '{serializableFolderType}'!");
                }
                if (fileDeserializationSubDelegates.ContainsKey(subSerializableFileType))
                {
                    throw new Exception($"The file deserialization delegate for file type '{subSerializableFileType}' is already registered as sub delegate for folder type '{serializableFolderType}'!");
                }

                fileSerializationSubDelegates.Add(subSerializableFileType, fileSerializationDelegates[subSerializableFileType]);
                fileDeserializationSubDelegates.Add(subSerializableFileType, fileDeserializationDelegates[subSerializableFileType]);
            }

            foreach (SerializableFolderTypeInfo subSerializableFolderTypeInfo in serializableFolderTypeInfo.SubSerializableFolderTypeInfos)
            {
                Type subSerializableFolderType = subSerializableFolderTypeInfo.SerializableFolderType;
                if (folderSerializationSubDelegates.ContainsKey(subSerializableFolderType))
                {
                    throw new Exception($"The folder serialization delegate for folder type '{subSerializableFolderType}' is already registered as sub delegate for folder type '{serializableFolderType}'!");
                }
                if (folderDeserializationSubDelegates.ContainsKey(subSerializableFolderType))
                {
                    throw new Exception($"The folder deserialization delegate for folder type '{subSerializableFolderType}' is already registered as sub delegate for folder type '{serializableFolderType}'!");
                }

                folderSerializationSubDelegates.Add(subSerializableFolderType, folderSerializationDelegates[subSerializableFolderType]);
                folderDeserializationSubDelegates.Add(subSerializableFolderType, folderDeserializationDelegates[subSerializableFolderType]);
            }

            fileSerializationSubDelegateDictionaries.Add(serializableFolderType, fileSerializationSubDelegates);
            fileDeserializationSubDelegateDictionaries.Add(serializableFolderType, fileDeserializationSubDelegates);

            folderSerializationSubDelegateDictionaries.Add(serializableFolderType, folderSerializationSubDelegates);
            folderDeserializationSubDelegateDictionaries.Add(serializableFolderType, folderDeserializationSubDelegates);
        }
        #endregion
    }
}
