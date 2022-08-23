namespace LooCast.Item
{
    using LooCast.Weapon;

    public abstract class WeaponItem : CountableItem
    {
        public abstract Weapon Weapon { get; protected set; }

        public WeaponItem(string name, int maxCount, int count, Weapon weapon) : base(name, maxCount, count)
        {
            Weapon = weapon;
        }
    }
}