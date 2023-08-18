using System;
using System.Linq;
using System.Diagnostics;
using System.Reflection;
using System.Collections.Generic;
using NLua;
using NLua.Exceptions;

namespace LooCast.System.Lua
{
    using LooCast.System;
    using LooCast.System.CSharp;
    using LooCast.System.ECS;

    public sealed class LuaManager : ModuleManager
    {
        #region Static Properties
        public static LuaManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<LuaManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static LuaManager instance;
        #endregion

        #region Fields
        private NLua.Lua lua;
        private Dictionary<string, LuaNamespaceInfo> registeredTopLevelLuaNamespaces;
        private Dictionary<string, LuaNamespaceInfo> newlyRegisteredTopLevelLuaNamespaces;
        #endregion

        #region Constructors
        public LuaManager() : base()
        {
            registeredTopLevelLuaNamespaces = new Dictionary<string, LuaNamespaceInfo>();
            newlyRegisteredTopLevelLuaNamespaces = new Dictionary<string, LuaNamespaceInfo>();

            lua = new NLua.Lua();
            lua.LoadCLRPackage();

            lua.DoString(@"
            LooCast = {}
            LooCast.System = {}
            LooCast.Universe = {}
            ");

            Type looCastApplicationType = typeof(LooCastApplication);
            lua.RegisterFunction("LooCast.Log", looCastApplicationType.GetMethod("Log"));

            Type universeManagerType = typeof(Universe.UniverseManager);
            lua.RegisterFunction("LooCast.Universe.CreateUniverse", universeManagerType.GetMethod("CreateUniverse"));
            lua.RegisterFunction("LooCast.Universe.CreateUniverseObserver", universeManagerType.GetMethod("CreateUniverseObserver"));
            lua.RegisterFunction("LooCast.Universe.SetUniverse", universeManagerType.GetMethod("SetUniverse"));
            lua.RegisterFunction("LooCast.Universe.SetUniverseObserver", universeManagerType.GetMethod("SetUniverseObserver"));
            lua.RegisterFunction("LooCast.Universe.GetUniverse", universeManagerType.GetMethod("GetUniverse"));
            lua.RegisterFunction("LooCast.Universe.GetUniverseObserver", universeManagerType.GetMethod("GetUniverseObserver"));

            Type universeType = typeof(Universe.Universe);
            lua.RegisterFunction("LooCast.Universe.GetChunkSize", universeType.GetMethod("GetChunkSize"));

            Type universeObserverType = typeof(Universe.UniverseObserver);
            lua.RegisterFunction("LooCast.Universe.GetObservingDistance", universeObserverType.GetMethod("GetObservingDistance"));

            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedLuaManagerEntityTypeName = typeof(LuaManager).AssemblyQualifiedName;
                string assemblyQualifiedLuaManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedLuaManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData instanceMetaData = new Entity.MetaData();
                instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedLuaManagerEntityTypeName;
                instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedLuaManagerEntityMetaDataTypeName;
                instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedLuaManagerEntityDataTypeName;
                instanceMetaData.EntityID = new Guid();

                Manager.Data instanceData = new Manager.Data();
                instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedLuaManagerEntityTypeName;
                instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedLuaManagerEntityMetaDataTypeName;
                instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedLuaManagerEntityDataTypeName;
                instanceData.ManagerName = "LuaManager";
                instanceData.ManagerParent = SystemManager.Instance;

                SetEntityMetaData(instanceMetaData);
                SetEntityData(instanceData);

                CSharpManager.OnTypesRegistered += (types) =>
                {
                    RegisterTypes(types);
                };

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                // Set pre-included components' metaData here

                // Set pre-included component's data here

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

        #region Static Methods
        public static void ExecuteLuaString(string luaString)
        {
            try
            {
                Instance.lua.DoString(luaString);
            }
            catch (LuaException ex)
            {
                LooCastApplication.Log($"[LuaManager] Error executing Lua code: {ex.Message}");
            }
        }
        #endregion

        #region Methods
        /// <summary>
        /// Registers a collection of types with the Lua manager.
        /// Analyzes each type, extracts the LuaNamespace and LuaMethod attributes,
        /// and registers the corresponding namespaces and methods with the Lua API.
        /// For performance optimization, it's recommended to register many types simultaneously.
        /// </summary>
        /// <param name="typeInfos">The types to be registered.</param>
        public void RegisterTypes(IEnumerable<TypeInfo> typeInfos)
        {
            Stopwatch stopwatch = new Stopwatch();
            stopwatch.Start();

            Dictionary<TypeInfo, LuaNamespaceAttribute> namespaceAttributes = GetNamespaceAttributes(typeInfos);
            Dictionary<int, Dictionary<TypeInfo, LuaNamespaceAttribute>> orderedNamespaceAttributeSets = OrderNamespaceAttributeSets(namespaceAttributes);
            Dictionary<string, List<LuaMethodAttribute>> methodAttributeSets = GetMethodAttributeSets(orderedNamespaceAttributeSets);

            RegisterNamespacesAndMethods(orderedNamespaceAttributeSets, methodAttributeSets);

            foreach (var kv in newlyRegisteredTopLevelLuaNamespaces)
            {
                if (registeredTopLevelLuaNamespaces.ContainsKey(kv.Key))
                {
                    throw new Exception($"[LuaManager] Attempted to register top-level Lua namespace '{kv.Key}' but it already exists!");
                }

                registeredTopLevelLuaNamespaces.Add(kv.Key, kv.Value);
            }
            newlyRegisteredTopLevelLuaNamespaces.Clear();

            stopwatch.Stop();
            UnityEngine.Debug.Log($"[LuaManager] Registering {typeInfos.Count()} types took {stopwatch.ElapsedMilliseconds}ms");
        }

        /// <summary>
        /// Retrieves all the <see cref="LuaNamespaceAttribute"/> from the provided <see cref="CSharp.TypeInfo"/> objects.
        /// </summary>
        /// <param name="typeInfos">The collection of <see cref="CSharp.TypeInfo"/> objects to extract the namespace attributes from.</param>
        /// <returns>A dictionary mapping the <see cref="CSharp.TypeInfo"/> objects to their respective <see cref="LuaNamespaceAttribute"/>.</returns>
        private Dictionary<TypeInfo, LuaNamespaceAttribute> GetNamespaceAttributes(IEnumerable<TypeInfo> typeInfos)
        {
            Dictionary<TypeInfo, LuaNamespaceAttribute> namespaceAttributes = new Dictionary<TypeInfo, LuaNamespaceAttribute>();
            foreach (TypeInfo typeInfo in typeInfos)
            {
                LuaNamespaceAttribute luaNamespaceAttribute = typeInfo.DirectAttributes.OfType<LuaNamespaceAttribute>().FirstOrDefault();
                if (luaNamespaceAttribute != null)
                {
                    namespaceAttributes.Add(typeInfo, luaNamespaceAttribute);
                }
            }
            return namespaceAttributes;
        }

        /// <summary>
        /// Orders the namespace attributes based on the number of parts in their namespace.
        /// </summary>
        /// <param name="namespaceAttributes">A dictionary mapping the <see cref="CSharp.TypeInfo"/> objects to their respective <see cref="LuaNamespaceAttribute"/>.</param>
        /// <returns>A dictionary containing the ordered namespace attribute sets.</returns>
        private Dictionary<int, Dictionary<TypeInfo, LuaNamespaceAttribute>> OrderNamespaceAttributeSets(Dictionary<TypeInfo, LuaNamespaceAttribute> namespaceAttributes)
        {
            Dictionary<int, Dictionary<TypeInfo, LuaNamespaceAttribute>> orderedNamespaceAttributeSets = new Dictionary<int, Dictionary<TypeInfo, LuaNamespaceAttribute>>();
            foreach (var kv in namespaceAttributes)
            {
                TypeInfo typeInfo = kv.Key;
                LuaNamespaceAttribute luaNamespaceAttribute = kv.Value;
                if (typeInfo.NamespaceParts != null && typeInfo.NamespaceParts.Length != 0)
                {
                    if (!orderedNamespaceAttributeSets.ContainsKey(typeInfo.NamespaceParts.Length))
                    {
                        orderedNamespaceAttributeSets.Add(typeInfo.NamespaceParts.Length, new Dictionary<TypeInfo, LuaNamespaceAttribute>());
                    }
                    orderedNamespaceAttributeSets[typeInfo.NamespaceParts.Length].Add(typeInfo, luaNamespaceAttribute);
                }
            }
            return orderedNamespaceAttributeSets;
        }

        /// <summary>
        /// Retrieves all the <see cref="LuaMethodAttribute"/> for each <see cref="LuaNamespaceAttribute"/> in the provided ordered namespace attribute sets.
        /// </summary>
        /// <param name="orderedNamespaceAttributeSets">The ordered namespace attribute sets.</param>
        /// <returns>A dictionary mapping the <see cref="LuaNamespaceAttribute"/> to their respective sets of <see cref="LuaMethodAttribute"/>.</returns>
        private Dictionary<string, List<LuaMethodAttribute>> GetMethodAttributeSets(Dictionary<int, Dictionary<TypeInfo, LuaNamespaceAttribute>> orderedNamespaceAttributeSets)
        {
            Dictionary<string, List<LuaMethodAttribute>> methodAttributeSets = new Dictionary<string, List<LuaMethodAttribute>>();
            foreach (var orderedNamespaceAttributeSet in orderedNamespaceAttributeSets.Values)
            {
                foreach (var kv in orderedNamespaceAttributeSet)
                {
                    TypeInfo typeInfo = kv.Key;
                    LuaNamespaceAttribute luaNamespaceAttribute = kv.Value;

                    IEnumerable<LuaMethodAttribute> methodAttributeSet = typeInfo.Methods.SelectMany(x => x.DirectAttributes.OfType<LuaMethodAttribute>());
                    if (methodAttributeSet != null && methodAttributeSet.Count() != 0)
                    {
                        if (!methodAttributeSets.ContainsKey(luaNamespaceAttribute.Namespace))
                        {
                            methodAttributeSets.Add(luaNamespaceAttribute.Namespace, new List<LuaMethodAttribute>());
                        }
                        methodAttributeSets[luaNamespaceAttribute.Namespace].AddRange(methodAttributeSet);
                    }
                }
            }
            return methodAttributeSets;
        }

        /// <summary>
        /// Registers the namespaces and methods in the Lua API based on the provided ordered namespace attribute sets and method attribute sets.
        /// </summary>
        /// <param name="orderedNamespaceAttributeSets">The ordered namespace attribute sets.</param>
        /// <param name="methodAttributeSets">The method attribute sets.</param>
        private void RegisterNamespacesAndMethods(Dictionary<int, Dictionary<TypeInfo, LuaNamespaceAttribute>> orderedNamespaceAttributeSets, Dictionary<string, List<LuaMethodAttribute>> methodAttributeSets)
        {
            foreach (var orderedNamespaceAttributeSet in orderedNamespaceAttributeSets.Values)
            {
                foreach (var kv in orderedNamespaceAttributeSet)
                {
                    TypeInfo typeInfo = kv.Key;
                    LuaNamespaceAttribute luaNamespaceAttribute = kv.Value;
                    RegisterNamespace(orderedNamespaceAttributeSet, typeInfo.NamespaceParts, 0, null);
                }
            }
        }

        /// <summary>
        /// Recursively registers a namespace based on the provided namespace parts, starting from the specified index.
        /// </summary>
        /// <param name="namespaceAttributes">A dictionary mapping the <see cref="CSharp.TypeInfo"/> objects to their respective <see cref="LuaNamespaceAttribute"/>.</param>
        /// <param name="namespaceParts">The parts of the namespace to register.</param>
        /// <param name="index">The index to start the registration from.</param>
        /// <param name="parent">The parent namespace information.</param>
        private void RegisterNamespace(Dictionary<TypeInfo, LuaNamespaceAttribute> namespaceAttributes, string[] namespaceParts, int index, LuaNamespaceInfo parent)
        {
            if (index >= namespaceParts.Length)
            {
                return;
            }

            // Get the current namespace part and construct the full namespace name
            string namespacePart = namespaceParts[index];
            string fullNamespace = string.Join(".", namespaceParts.Take(index + 1));

            // Check if the namespace is already registered
            if (!newlyRegisteredTopLevelLuaNamespaces.ContainsKey(fullNamespace))
            {
                // Create a new LuaNamespaceInfo
                LuaNamespaceInfo namespaceInfo = new LuaNamespaceInfo(parent != null ? new[] { parent } : null, namespacePart);

                // Register the namespace in the Lua API
                if (parent == null)
                {
                    lua.DoString($"{fullNamespace} = {{}}");
                }
                else
                {
                    lua.DoString($"{parent.Namespace}.{namespacePart} = {{}}");
                }

                // Register methods in the namespace
                foreach (var kv in namespaceAttributes)
                {
                    TypeInfo typeInfo = kv.Key;
                    LuaNamespaceAttribute luaNamespaceAttribute = kv.Value;
                    if (typeInfo.Namespace == fullNamespace)
                    {
                        foreach (var methodInfo in typeInfo.Methods)
                        {
                            var luaMethodAttribute = methodInfo.DirectAttributes.OfType<LuaMethodAttribute>().FirstOrDefault();
                            if (luaMethodAttribute != null)
                            {
                                namespaceInfo.Methods.Add(luaMethodAttribute.MethodName, new LuaMethodInfo(namespaceInfo, luaMethodAttribute.MethodName, methodInfo.Method));
                                lua.RegisterFunction($"{fullNamespace}.{luaMethodAttribute.MethodName}", methodInfo.Method);
                            }
                        }
                    }
                }

                // Recursively register child namespaces
                RegisterNamespace(namespaceAttributes, namespaceParts, index + 1, namespaceInfo);
            }
            else
            {
                // If the namespace is already registered, continue with the child namespaces
                RegisterNamespace(namespaceAttributes, namespaceParts, index + 1, newlyRegisteredTopLevelLuaNamespaces[fullNamespace]);
            }
        }
        #endregion
    }
}