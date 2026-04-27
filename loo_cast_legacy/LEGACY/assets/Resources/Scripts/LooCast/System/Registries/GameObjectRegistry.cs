namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class GameObjectRegistry : Registry<GameObjectIdentifier, GameObject>
    {
        public GameObjectRegistry() : base("LooCast.System.Registries.GameObjectRegistry")
        {
            
        }
    }
}
