using System;
using System.IO;
using UnityEngine;

namespace LooCast.Game
{
    using Core;

    [Serializable]
    public class PersistentData<ObjectType> where ObjectType : IGameDataHandler
    {
        [SerializeField] private string jsonSerializedData;
        [SerializeField] private ExtendedMonoBehaviour.InstanceIdentifier identifier;

        public PersistentData(ObjectType _object)
        {
            // Get Persistent Data from Object
        }

        public PersistentData<ObjectType> CreatePersistentDataFromGameObject(ObjectType _object)
        {
            RuntimeData gameData = ((IGameDataHandler)_object).GetData();
        }

        public RuntimeData CreateGameDataFromPersistentData(PersistentData<ObjectType> persistentData)
        {
            ((IGameDataHandler)_object).SetData();
        }

        public void LoadDataIntoGameObject
        {

        }

        public void CreateGameObjectFromData(PersistentData<ObjectType> persistentData)
        {
            persistentData.LoadObject();
        }

        public GameObject LoadGameObject()
        {
            GameObject loadedGameObject = GameObject.Instantiate(Resources.Load<GameObject>(objectPrefabPath));
            ObjectType loadedObject = loadedGameObject.GetComponent<ObjectType>();
            if (loadedObject == null)
            {
                throw new Exception($"Could not find a component of type '{typeof(ObjectType).FullName}' attached to loaded GameObject '{loadedGameObject.name}'");
            }
            loadedObject.SetData(new RuntimeData(jsonSerializedData, ));
            return loadedGameObject;
        }

        public void SaveObject(GameObject gameObject)
        {

        }
    }
}
