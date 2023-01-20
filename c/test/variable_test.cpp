#include <gtest/gtest.h>

#include "variable/variable.hpp"
#include "object/object.hpp"

TEST(Variable, Basic_test)
{
    Variable val1 = test::VariableTest();
    Variable val2 = test::VariableTest();

    EXPECT_EQ(val1.get_flag(), Type::Undefined);
    EXPECT_EQ(val2.get_flag(), Type::Undefined);

    val1.set_undefined();
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Undefined);
    EXPECT_EQ(val2.get_flag(), Type::Undefined);

    val1.set_null();
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Null);
    EXPECT_EQ(val2.get_flag(), Type::Null);

    val1.set_number(Number(13));
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Number);
    EXPECT_EQ(val2.get_flag(), Type::Number);
    EXPECT_EQ(val1.get_number(), Number(13));
    EXPECT_EQ(val2.get_number(), Number(13));

    val1.set_boolean(true);
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Boolean);
    EXPECT_EQ(val2.get_flag(), Type::Boolean);
    EXPECT_EQ(val1.get_boolean(), true);
    EXPECT_EQ(val2.get_boolean(), true);

    val1.set_string("foo");
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::String);
    EXPECT_EQ(val2.get_flag(), Type::String);
    EXPECT_EQ(val1.get_string(), "foo");
    EXPECT_EQ(val2.get_string(), "foo");

    val1.set_object(Object());
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Object);
    EXPECT_EQ(val2.get_flag(), Type::Object);
    EXPECT_EQ(val1.get_object(), Object());
    EXPECT_EQ(val2.get_object(), Object());

    Variable *tmp1 = GarbageCollector<Variable>::get_instance().allocate();
    Variable *tmp2 = GarbageCollector<Variable>::get_instance().allocate();
    val1.set_array(Array({tmp1, tmp2}));
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Array);
    EXPECT_EQ(val2.get_flag(), Type::Array);
    EXPECT_EQ(val1.get_array().len(), 2);
    EXPECT_EQ(val2.get_array().len(), 2);
}

TEST(Variable, to_boolean_test)
{
    EXPECT_EQ(test::VariableTest().to_boolean(), false);
    EXPECT_EQ(test::VariableTest().set_null().to_boolean(), false);
    EXPECT_EQ(test::VariableTest().set_number(Number(1)).to_boolean(), true);
    EXPECT_EQ(test::VariableTest().set_boolean(true).to_boolean(), true);
    EXPECT_EQ(test::VariableTest().set_boolean(false).to_boolean(), false);
    EXPECT_EQ(test::VariableTest().set_string("Hello world").to_boolean(), true);
    EXPECT_EQ(test::VariableTest().set_string("").to_boolean(), false);
    EXPECT_EQ(test::VariableTest().set_object(Object()).to_boolean(), false);
    EXPECT_EQ(test::VariableTest().set_array(Array()).to_boolean(), false);
}

TEST(Variable, to_number_test)
{
    EXPECT_EQ(test::VariableTest().to_number(), Number(NumberType::NaN));
    EXPECT_EQ(test::VariableTest().set_null().to_number(), Number(0));
    EXPECT_EQ(test::VariableTest().set_number(Number(13)).to_number(), Number(13));
    EXPECT_EQ(test::VariableTest().set_boolean(true).to_number(), Number(1));
    EXPECT_EQ(test::VariableTest().set_boolean(false).to_number(), Number(0));
    EXPECT_EQ(test::VariableTest().set_string("Hello world").to_number(), Number(NumberType::NaN));
    EXPECT_EQ(test::VariableTest().set_string("").to_number(), Number(NumberType::NaN));
    EXPECT_EQ(test::VariableTest().set_object(Object()).to_number(), Number(NumberType::NaN));
    EXPECT_EQ(test::VariableTest().set_array(Array()).to_number(), Number(NumberType::NaN));
}

TEST(Variable, to_string_test)
{
    EXPECT_EQ(test::VariableTest().to_string(), "undefined");
    EXPECT_EQ(test::VariableTest().set_null().to_string(), "null");
    EXPECT_EQ(test::VariableTest().set_number(Number(NumberType::NaN)).to_string(), "NaN");
    EXPECT_EQ(test::VariableTest().set_boolean(true).to_string(), "true");
    EXPECT_EQ(test::VariableTest().set_boolean(false).to_string(), "false");
    EXPECT_EQ(test::VariableTest().set_string("Hello world").to_string(), "Hello world");
    EXPECT_EQ(test::VariableTest().set_string("").to_string(), "");
    EXPECT_EQ(test::VariableTest().set_object(Object()).to_string(), "{}");
    EXPECT_EQ(test::VariableTest().set_array(Array()).to_string(), "[]");
}

TEST(Variable, arithmetic_test)
{
    Variable res = test::VariableTest();

    // "Hello " + "world" = "Hello world"
    res = test::VariableTest().set_string("Hello ") + test::VariableTest().set_string("world");
    EXPECT_EQ(res.get_flag(), Type::String);
    EXPECT_EQ(res.get_string(), "Hello world");

    // 2 + " world" = "2.000000 world"
    res = test::VariableTest().set_number(Number(2)) + test::VariableTest().set_string(" world");
    EXPECT_EQ(res.get_flag(), Type::String);
    EXPECT_EQ(res.get_string(), "2.000000 world");

    // "Hello " + 2 = "Hello 2.000000"
    res = test::VariableTest().set_string("Hello ") + test::VariableTest().set_number(Number(2));
    EXPECT_EQ(res.get_flag(), Type::String);
    EXPECT_EQ(res.get_string(), "Hello 2.000000");

    res = test::VariableTest().set_number(Number(1)) + test::VariableTest().set_number(Number(2));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(3));

    res = test::VariableTest().set_number(Number(1)) - test::VariableTest().set_number(Number(2));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(-1));

    res = test::VariableTest().set_number(Number(1)) * test::VariableTest().set_number(Number(2));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(2));

    res = test::VariableTest().set_number(Number(2)) / test::VariableTest().set_number(Number(1));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(2));
}

TEST(Variable, logical_not_test)
{
    EXPECT_EQ(!test::VariableTest(), true);
    EXPECT_EQ(!test::VariableTest().set_null(), true);
    EXPECT_EQ(!test::VariableTest().set_boolean(true), false);
    EXPECT_EQ(!test::VariableTest().set_boolean(false), true);
    EXPECT_EQ(!test::VariableTest().set_number(Number(2.5)), false);
    EXPECT_EQ(!test::VariableTest().set_number(Number(0)), true);
    EXPECT_EQ(!test::VariableTest().set_string("Hello world"), false);
    EXPECT_EQ(!test::VariableTest().set_string(""), true);
    EXPECT_EQ(!test::VariableTest().set_object(Object()), true);
    EXPECT_EQ(!test::VariableTest().set_array(Array()), true);
}

TEST(Variable, logical_and_test)
{
    Variable res = test::VariableTest();

    res = test::VariableTest().set_boolean(true) && test::VariableTest().set_boolean(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = test::VariableTest().set_boolean(false) && test::VariableTest().set_boolean(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);

    res = test::VariableTest().set_boolean(true) && test::VariableTest().set_boolean(false);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);

    res = test::VariableTest().set_boolean(false) && test::VariableTest().set_boolean(false);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);

    res = test::VariableTest().set_number(Number(11)) && test::VariableTest().set_boolean(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = test::VariableTest().set_boolean(true) && test::VariableTest().set_number(Number(11));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(11));

    res = test::VariableTest().set_number(Number(0)) && test::VariableTest().set_boolean(true);
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(0));

    res = test::VariableTest().set_boolean(true) && test::VariableTest().set_number(Number(0));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(0));

    res = test::VariableTest().set_boolean(false) && test::VariableTest().set_number(Number(0));
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);

    res = test::VariableTest().set_number(Number(0)) && test::VariableTest().set_boolean(false);
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(0));
}

TEST(Variable, logical_or_test)
{
    Variable res = test::VariableTest();

    res = test::VariableTest().set_boolean(true) || test::VariableTest().set_boolean(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = test::VariableTest().set_boolean(false) || test::VariableTest().set_boolean(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = test::VariableTest().set_boolean(true) || test::VariableTest().set_boolean(false);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = test::VariableTest().set_boolean(false) || test::VariableTest().set_boolean(false);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);

    res = test::VariableTest().set_number(Number(11)) || test::VariableTest().set_boolean(true);
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(11));

    res = test::VariableTest().set_boolean(true) || test::VariableTest().set_number(Number(11));
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = test::VariableTest().set_number(Number(0)) || test::VariableTest().set_boolean(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = test::VariableTest().set_boolean(true) || test::VariableTest().set_number(Number(0));
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = test::VariableTest().set_boolean(false) || test::VariableTest().set_number(Number(0));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(0));

    res = test::VariableTest().set_number(Number(0)) || test::VariableTest().set_boolean(false);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);
}

TEST(Variable, logical_eq_test)
{
    EXPECT_TRUE(test::VariableTest() == test::VariableTest());
    EXPECT_TRUE(test::VariableTest().set_null() == test::VariableTest().set_null());
    EXPECT_TRUE(test::VariableTest().set_boolean(true) == test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_boolean(false) == test::VariableTest().set_boolean(false));
    EXPECT_TRUE(test::VariableTest().set_number(Number(13)) == test::VariableTest().set_number(Number(13)));
    EXPECT_TRUE(test::VariableTest().set_string("Hello world") == test::VariableTest().set_string("Hello world"));
    EXPECT_TRUE(test::VariableTest().set_object(Object()) != test::VariableTest().set_object(Object()));
    EXPECT_TRUE(test::VariableTest().set_array(Array()) != test::VariableTest().set_array(Array()));

    EXPECT_FALSE(test::VariableTest() != test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_null() != test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_boolean(true) != test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) != test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_number(Number(13)) != test::VariableTest().set_number(Number(13)));
    EXPECT_FALSE(test::VariableTest().set_string("Hello world") != test::VariableTest().set_string("Hello world"));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) == test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) == test::VariableTest().set_array(Array()));
}

TEST(Variable, logical_cmp_test)
{
    // > tests
    EXPECT_TRUE(test::VariableTest().set_number(Number(2)) > test::VariableTest().set_number(Number(0)));
    EXPECT_TRUE(test::VariableTest().set_number(Number(2)) > test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_boolean(true) > test::VariableTest().set_number(Number(0)));
    EXPECT_TRUE(test::VariableTest().set_boolean(true) > test::VariableTest().set_number(Number(-1)));
    EXPECT_TRUE(test::VariableTest().set_number(Number(1)) > test::VariableTest().set_null());
    EXPECT_TRUE(test::VariableTest().set_boolean(true) > test::VariableTest().set_null());
    EXPECT_TRUE(test::VariableTest().set_string("b") > test::VariableTest().set_string("a"));

    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) > test::VariableTest());
    EXPECT_FALSE(test::VariableTest() > test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) > test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_string("Hello") > test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) > test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) > test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) > test::VariableTest().set_array(Array()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) > test::VariableTest().set_number(Number(2)));

    EXPECT_FALSE(test::VariableTest().set_boolean(true) > test::VariableTest());
    EXPECT_FALSE(test::VariableTest() > test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) > test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_string("Hello") > test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) > test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) > test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) > test::VariableTest().set_array(Array()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) > test::VariableTest().set_boolean(true));

    EXPECT_FALSE(test::VariableTest().set_boolean(false) > test::VariableTest());
    EXPECT_FALSE(test::VariableTest() > test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) > test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_string("Hello") > test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) > test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) > test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) > test::VariableTest().set_array(Array()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) > test::VariableTest().set_boolean(false));

    EXPECT_FALSE(test::VariableTest().set_null() > test::VariableTest());
    EXPECT_FALSE(test::VariableTest() > test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() > test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_string("Hello") > test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() > test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) > test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() > test::VariableTest().set_array(Array()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) > test::VariableTest().set_null());

    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) > test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) > test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) > test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_string("Hello") > test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_null() > test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest() > test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_object(Object()) > test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) > test::VariableTest().set_array(Array()));

    // >= tests
    EXPECT_TRUE(test::VariableTest().set_number(Number(2)) >= test::VariableTest().set_number(Number(0)));
    EXPECT_TRUE(test::VariableTest().set_number(Number(2)) >= test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_boolean(true) >= test::VariableTest().set_number(Number(0)));
    EXPECT_TRUE(test::VariableTest().set_boolean(true) >= test::VariableTest().set_number(Number(-1)));
    EXPECT_TRUE(test::VariableTest().set_number(Number(1)) >= test::VariableTest().set_null());
    EXPECT_TRUE(test::VariableTest().set_boolean(true) >= test::VariableTest().set_null());
    EXPECT_TRUE(test::VariableTest().set_string("b") >= test::VariableTest().set_string("a"));
    EXPECT_TRUE(test::VariableTest().set_number(Number(2)) >= test::VariableTest().set_number(Number(2)));
    EXPECT_TRUE(test::VariableTest().set_boolean(true) >= test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_boolean(false) >= test::VariableTest().set_boolean(false));
    EXPECT_TRUE(test::VariableTest().set_string("Hello") >= test::VariableTest().set_string("Hello"));
    EXPECT_TRUE(test::VariableTest().set_null() >= test::VariableTest().set_null());

    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) >= test::VariableTest());
    EXPECT_FALSE(test::VariableTest() >= test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) >= test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_string("Hello") >= test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) >= test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) >= test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) >= test::VariableTest().set_array(Array()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) >= test::VariableTest().set_number(Number(2)));

    EXPECT_FALSE(test::VariableTest().set_boolean(true) >= test::VariableTest());
    EXPECT_FALSE(test::VariableTest() >= test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) >= test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_string("Hello") >= test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) >= test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) >= test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) >= test::VariableTest().set_array(Array()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) >= test::VariableTest().set_boolean(true));

    EXPECT_FALSE(test::VariableTest().set_boolean(false) >= test::VariableTest());
    EXPECT_FALSE(test::VariableTest() >= test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) >= test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_string("Hello") >= test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) >= test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) >= test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) >= test::VariableTest().set_array(Array()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) >= test::VariableTest().set_boolean(false));

    EXPECT_FALSE(test::VariableTest().set_null() >= test::VariableTest());
    EXPECT_FALSE(test::VariableTest() >= test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() >= test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_string("Hello") >= test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() >= test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) >= test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() >= test::VariableTest().set_array(Array()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) >= test::VariableTest().set_null());

    EXPECT_FALSE(test::VariableTest() >= test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_object(Object()) >= test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) >= test::VariableTest().set_array(Array()));

    // < tests
    EXPECT_TRUE(test::VariableTest().set_number(Number(0)) < test::VariableTest().set_number(Number(2)));
    EXPECT_TRUE(test::VariableTest().set_boolean(true) < test::VariableTest().set_number(Number(2)));
    EXPECT_TRUE(test::VariableTest().set_number(Number(0)) < test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_number(Number(-1)) < test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_null() < test::VariableTest().set_number(Number(1)));
    EXPECT_TRUE(test::VariableTest().set_null() < test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_string("a") < test::VariableTest().set_string("b"));

    EXPECT_FALSE(test::VariableTest() < test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) < test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_string("Hello") < test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) < test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) < test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) < test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) < test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) < test::VariableTest().set_array(Array()));

    EXPECT_FALSE(test::VariableTest() < test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) < test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_string("Hello") < test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) < test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) < test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) < test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) < test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) < test::VariableTest().set_array(Array()));

    EXPECT_FALSE(test::VariableTest() < test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) < test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_string("Hello") < test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) < test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) < test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) < test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) < test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) < test::VariableTest().set_array(Array()));

    EXPECT_FALSE(test::VariableTest() < test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() < test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_string("Hello") < test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() < test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) < test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() < test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) < test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() < test::VariableTest().set_array(Array()));

    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) < test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) < test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) < test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_string("Hello") < test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_null() < test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest() < test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_object(Object()) < test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) < test::VariableTest().set_array(Array()));

    // <= tests
    EXPECT_TRUE(test::VariableTest().set_number(Number(0)) <= test::VariableTest().set_number(Number(2)));
    EXPECT_TRUE(test::VariableTest().set_boolean(true) <= test::VariableTest().set_number(Number(2)));
    EXPECT_TRUE(test::VariableTest().set_number(Number(0)) <= test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_number(Number(-1)) <= test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_null() <= test::VariableTest().set_number(Number(1)));
    EXPECT_TRUE(test::VariableTest().set_null() <= test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_string("a") <= test::VariableTest().set_string("b"));
    EXPECT_TRUE(test::VariableTest().set_number(Number(2)) <= test::VariableTest().set_number(Number(2)));
    EXPECT_TRUE(test::VariableTest().set_boolean(true) <= test::VariableTest().set_boolean(true));
    EXPECT_TRUE(test::VariableTest().set_boolean(false) <= test::VariableTest().set_boolean(false));
    EXPECT_TRUE(test::VariableTest().set_string("Hello") <= test::VariableTest().set_string("Hello"));
    EXPECT_TRUE(test::VariableTest().set_null() <= test::VariableTest().set_null());

    EXPECT_FALSE(test::VariableTest() <= test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) <= test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_string("Hello") <= test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) <= test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) <= test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) <= test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) <= test::VariableTest().set_number(Number(2)));
    EXPECT_FALSE(test::VariableTest().set_number(Number(2)) <= test::VariableTest().set_array(Array()));

    EXPECT_FALSE(test::VariableTest() <= test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) <= test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_string("Hello") <= test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) <= test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) <= test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) <= test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) <= test::VariableTest().set_boolean(true));
    EXPECT_FALSE(test::VariableTest().set_boolean(true) <= test::VariableTest().set_array(Array()));

    EXPECT_FALSE(test::VariableTest() <= test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) <= test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_string("Hello") <= test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) <= test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) <= test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) <= test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) <= test::VariableTest().set_boolean(false));
    EXPECT_FALSE(test::VariableTest().set_boolean(false) <= test::VariableTest().set_array(Array()));

    EXPECT_FALSE(test::VariableTest() <= test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() <= test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_string("Hello") <= test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() <= test::VariableTest().set_string("Hello"));
    EXPECT_FALSE(test::VariableTest().set_object(Object()) <= test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() <= test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) <= test::VariableTest().set_null());
    EXPECT_FALSE(test::VariableTest().set_null() <= test::VariableTest().set_array(Array()));

    EXPECT_FALSE(test::VariableTest() < test::VariableTest());
    EXPECT_FALSE(test::VariableTest().set_object(Object()) < test::VariableTest().set_object(Object()));
    EXPECT_FALSE(test::VariableTest().set_array(Array()) < test::VariableTest().set_array(Array()));
}