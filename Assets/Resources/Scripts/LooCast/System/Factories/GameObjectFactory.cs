namespace LooCast.System.Factories
{
    using global::LooCast.System.MetaData;

    public class GameObjectFactory : Factory<GameObject, GameObjectMetaData>
    {
        public override GameObject CreateInstance(GameObjectMetaData metadata)
        {
            return new GameObject(metadata);
        }
    }
}